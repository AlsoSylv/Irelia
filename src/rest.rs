use futures_util::StreamExt;
use hyper::{client::HttpConnector, header::AUTHORIZATION, Request};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    utils::{
        process_info::get_port_and_auth,
        request::{request_template, setup_hyper_client, uri_builder},
    },
    Error,
};

pub struct LCUClient {
    url: String,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    auth_header: String,
}

pub enum RequestType {
    Get,
    Post,
    Put,
    Delete,
    Head,
}

impl LCUClient {
    /// Makes a connection to the LCU, errors if it is not found or not running
    pub fn new() -> Result<Self, Error> {
        let port_pass = get_port_and_auth()?;
        let client = setup_hyper_client().unwrap();
        Ok(Self {
            url: format!("127.0.0.1:{}", port_pass.0),
            client,
            auth_header: format!("Basic {}", port_pass.1),
        })
    }

    /// Make a get request to the LCU
    ///
    /// # Example:
    /// ```rust
    /// use irelia::rest::LCUClient;
    /// use serde_json::Value;
    ///
    /// async fn get_summoner() {
    ///     let client = LCUClient::new().unwrap();
    ///     let summoner: Option<Value> = client.get("/lol-summoner/v1/current-summoner").await.unwrap();
    /// }
    /// ```
    pub async fn get<R>(&self, endpoint: &str) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.lcu_template::<(), R>(endpoint, "GET", None).await
    }

    /// Make a post request to the LCU, with any body that implements serde::Serialize
    pub async fn post<T, R>(&self, endpoint: &str, body: T) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.lcu_template(endpoint, "POST", Some(body)).await
    }

    /// Make a put request to the LCU, with any body that implements serde::Serialize
    pub async fn put<T, R>(&self, endpoint: &str, body: T) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.lcu_template(endpoint, "PUT", Some(body)).await
    }

    /// Make a delete request to the LCU
    pub async fn delete<R>(&self, endpoint: &str) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.lcu_template::<(), R>(endpoint, "DELETE", None).await
    }

    /// Make a head request to the LCU
    pub async fn head<R>(&self, endpoint: &str) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.lcu_template::<(), R>(endpoint, "HEAD", None).await
    }

    /// Make a patch request to the LCU, with any body that implements serde::Serialize
    pub async fn patch<T, R>(&self, endpoint: &str, body: T) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.lcu_template(endpoint, "PATCH", Some(body)).await
    }

    pub async fn batched(
        &self,
        requests: &[(RequestType, &str, Option<Value>)],
        buffer_size: usize,
    ) -> Vec<Result<Option<Value>, Error>> {
        let returns = futures_util::stream::iter(requests.iter().map(
            |(request_type, endpoint, body)| async move {
                match request_type {
                    RequestType::Get => self.get(endpoint).await,
                    RequestType::Post => self.post(endpoint, body.as_ref().unwrap()).await,
                    RequestType::Put => self.put(endpoint, body.as_ref().unwrap()).await,
                    RequestType::Delete => self.delete(endpoint).await,
                    RequestType::Head => self.head(endpoint).await,
                }
            },
        ))
        .buffered(buffer_size)
        .collect();

        returns.await
    }

    async fn lcu_template<T, R>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<T>,
    ) -> Result<Option<R>, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let uri = uri_builder(&self.url, endpoint)?;

        let body = match body {
            Some(body) => {
                let Ok(json) = serde_json::value::to_value(body) else {
                    return Err(Error::InvalidBody);
                };
                hyper::Body::from(json.to_string())
            }
            None => hyper::Body::empty(),
        };

        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header(AUTHORIZATION, &self.auth_header)
            .body(body);

        request_template::<Option<R>>(Error::LCUProcessNotRunning, req, &self.client, |bytes| {
            if bytes.is_empty() {
                Ok(None)
            } else {
                serde_json::from_slice::<R>(&bytes)
                    .map_or(Err(Error::FailedParseJson), |value| Ok(Some(value)))
            }
        })
        .await
    }
}
