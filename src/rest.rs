use hyper::{client::HttpConnector, header::AUTHORIZATION, http::uri, Request};
use hyper_tls::HttpsConnector;
use serde::Serialize;
use serde_json::Value;

use crate::{
    utils::{process_info::get_port_and_auth, request::setup_hyper_client},
    Errors,
};

pub struct LCUClient {
    url: String,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    auth_header: String,
}

impl LCUClient {
    pub fn new() -> Result<Self, Errors> {
        let port_pass = get_port_and_auth().map_err(Errors::ProcessInfoError)?;
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
    ///
    /// async fn get_summoner() {
    ///     let client = LCUClient::new().unwrap();
    ///     let summoner = client.get("/lol-summoner/v1/current-summoner").await.unwrap();
    /// }
    /// ```
    pub async fn get(&self, endpoint: &str) -> Result<Option<Value>, Errors> {
        self.template::<()>(endpoint, "GET", None).await
    }

    pub async fn post<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<Option<Value>, Errors> {
        self.template(endpoint, "POST", Some(body)).await
    }

    pub async fn put<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<Option<Value>, Errors> {
        self.template(endpoint, "PUT", Some(body)).await
    }

    pub async fn delete(&self, endpoint: &str) -> Result<Option<Value>, Errors> {
        self.template::<()>(endpoint, "DELETE", None).await
    }

    async fn template<T: Serialize>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<T>,
    ) -> Result<Option<Value>, Errors> {
        let Ok(uri) = uri::Builder::new().scheme("https").authority(self.url.as_bytes()).path_and_query(endpoint).build() else {
            return Err(Errors::InvalidRequest);
        };

        let body = match body {
            Some(body) => {
                let Ok(json) = serde_json::value::to_value(body) else {
                    return Err(Errors::InvalidBody);
                };
                hyper::Body::from(json.to_string())
            }
            None => hyper::Body::empty(),
        };

        let Ok(req) = Request::builder()
            .method(method)
            .uri(uri)
            .header(AUTHORIZATION, &self.auth_header)
            .body(body)
        else {
            return Err(Errors::InvalidRequest);
        };

        match self.client.request(req).await {
            Ok(mut res) => {
                let body = res.body_mut();
                let bytes = hyper::body::to_bytes(body).await;
                match bytes {
                    Ok(bytes) => {
                        if bytes.is_empty() {
                            Ok(None)
                        } else {
                            serde_json::from_slice::<Value>(&bytes)
                                .map_or(Err(Errors::FailedParseJson), |value| Ok(Some(value)))
                        }
                    }
                    Err(err) => panic!("{}", err),
                }
            }
            Err(err) => {
                if err.is_connect() {
                    Err(Errors::LCUStoppedRunning)
                } else {
                    panic!("{}", err)
                }
            }
        }
    }
}
