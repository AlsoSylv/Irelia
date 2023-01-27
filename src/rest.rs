use hyper::{client::HttpConnector, header::AUTHORIZATION, Request};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

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

impl LCUClient {
    pub fn new() -> Result<Self, Error> {
        let port_pass = get_port_and_auth()?;
        let client = setup_hyper_client().unwrap();
        Ok(Self {
            url: format!("127.0.0.1:{}", port_pass.0),
            client,
            auth_header: format!("Basic {}", port_pass.1),
        })
    }

    /// Function to get a response from a specified endpoint
    ///
    /// ```rust
    /// use samira::rest::LCUClient;
    /// use serde_json::Value;
    ///
    /// async fn get_summoner() {
    ///     let client = LCUClient::new().unwrap();
    ///     let summoner: Option<Value> = client.get("/lol-summoner/v1/current-summoner").await.unwrap();
    /// }
    /// ```
    pub async fn get<R: DeserializeOwned>(&self, endpoint: &str) -> Result<Option<R>, Error> {
        self.lcu_template::<(), R>(endpoint, "GET", None).await
    }

    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<Option<R>, Error> {
        self.lcu_template(endpoint, "POST", Some(body)).await
    }

    pub async fn put<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<Option<R>, Error> {
        self.lcu_template(endpoint, "PUT", Some(body)).await
    }

    pub async fn delete<R: DeserializeOwned>(&self, endpoint: &str) -> Result<Option<R>, Error> {
        self.lcu_template::<(), R>(endpoint, "DELETE", None).await
    }

    pub async fn head<R: DeserializeOwned>(&self, endpoint: &str) -> Result<Option<R>, Error> {
        self.lcu_template::<(), R>(endpoint, "HEAD", None).await
    }

    pub async fn patch<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<Option<R>, Error> {
        self.lcu_template(endpoint, "PATCH", Some(body)).await
    }

    async fn lcu_template<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<T>,
    ) -> Result<Option<R>, Error> {
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
