use hyper::{client::HttpConnector, header::AUTHORIZATION, Request};
use hyper_tls::HttpsConnector;
use serde::Serialize;
use serde_json::Value;

use crate::{
    utils::{
        process_info::{get_port_and_auth, get_running_client},
        request::setup_hyper_client,
    },
    Errors,
};

pub struct LCUClient {
    port: String,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    auth_header: String,
}

impl LCUClient {
    pub fn new() -> Result<Self, Errors> {
        let process = get_running_client().map_err(|err| Errors::ProcessInfoError(err))?;
        let port_pass = get_port_and_auth(process).map_err(|err| Errors::ProcessInfoError(err))?;
        let client = setup_hyper_client().unwrap();
        Ok(Self {
            port: port_pass.0,
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
        let Ok(uri) =
            format!("https://127.0.0.1:{}{}", self.port, endpoint).parse::<hyper::Uri>() 
        else {
            return Err(Errors::InvalidRequest);
        };

        let Ok(req) = Request::builder()
            .method("GET")
            .uri(uri)
            .header(AUTHORIZATION, &self.auth_header)
            .body(hyper::Body::empty())
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

    pub async fn post<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<Option<Value>, Errors> {
        let Ok(json) = serde_json::value::to_value(body) else {
            return Err(Errors::InvalidBody);
        };

        let Ok(uri) =
            format!("https://127.0.0.1:{}{}", self.port, endpoint).parse::<hyper::Uri>() 
        else {
            return Err(Errors::InvalidRequest);
        };

        let Ok(req) = Request::builder()
            .method("POST")
            .uri(uri)
            .header(AUTHORIZATION, &self.auth_header)
            .body(hyper::Body::from(json.to_string()))
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

    pub async fn put<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<Option<Value>, Errors> {
        let Ok(json) = serde_json::value::to_value(body) else {
            return Err(Errors::InvalidBody);
        };

        let Ok(uri) =
            format!("https://127.0.0.1:{}{}", self.port, endpoint).parse::<hyper::Uri>() 
        else {
            return Err(Errors::InvalidRequest);
        };

        let Ok(req) = Request::builder()
            .method("PUT")
            .uri(uri)
            .header(AUTHORIZATION, &self.auth_header)
            .body(hyper::Body::from(json.to_string()))
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

    pub async fn delete(&self, endpoint: &str) -> Result<Option<Value>, Errors> {
        let Ok(uri) =
            format!("https://127.0.0.1:{}{}", self.port, endpoint).parse::<hyper::Uri>() 
        else {
            return Err(Errors::InvalidRequest);
        };

        let Ok(req) = Request::builder()
            .method("DELETE")
            .uri(uri)
            .header(AUTHORIZATION, &self.auth_header)
            .body(hyper::Body::empty())
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
