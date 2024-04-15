use crate::LCUError;

use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::header::AUTHORIZATION;
use hyper::http::uri;
use hyper::{Request, Response};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::setup_tls::setup_tls_connector;

/// Struct that represents any connection to the in game or rest APIs, this client has to be constructed and then passed to the clients
///
/// # Example
/// ```rs
/// use irelia::{RequestClient, rest::LCUClient};
///
/// fn main() {
///     let client = RequestClient::new();
///     
///     let lcu_client = LCUClient::new();
/// }
/// ```
pub struct RequestClient {
    client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
}

impl RequestClient {
    #[must_use]
    /// Creates a client to be passed to the LCU and in game structs
    pub fn new() -> RequestClient {
        let tls = setup_tls_connector();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_or_http()
            .enable_http1()
            .build();
        let client =
            Client::builder(hyper_util::rt::TokioExecutor::new()).build::<_, Full<Bytes>>(https);

        RequestClient { client }
    }

    pub(crate) async fn raw_request_template<T>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
    ) -> Result<Response<Incoming>, LCUError>
    where
        T: Serialize,
    {
        let built_uri = uri::Builder::new()
            .scheme("https")
            .authority(url.as_bytes())
            .path_and_query(endpoint)
            .build()
            .map_or_else(|err| Err(LCUError::HyperHttpError(err)), Ok)?;

        let body = match body {
            Some(body) => match serde_json::value::to_value(body) {
                Ok(json) => Ok(http_body_util::Full::from(json.to_string())),
                Err(err) => Err(LCUError::SerdeJsonError(err)),
            },
            None => Ok(Full::new(Bytes::new())),
        }?;

        let request = match auth_header {
            Some(header) => Request::builder()
                .method(method)
                .uri(built_uri)
                .header(AUTHORIZATION, header)
                .body(body),
            None => Request::builder().method(method).uri(built_uri).body(body),
        }
        .map_err(LCUError::HyperHttpError)?;

        self.client
            .request(request)
            .await
            .map_err(LCUError::HyperClientError)
    }

    pub(crate) async fn request_template<T, R>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
        return_logic: fn(bytes: Bytes) -> Result<R, LCUError>,
    ) -> Result<R, LCUError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let mut response = self
            .raw_request_template(url, endpoint, method, body, auth_header)
            .await?;

        let body = response.body_mut();

        let bytes = body
            .collect()
            .await
            .map_err(LCUError::HyperError)?
            .to_bytes();

        return_logic(bytes)
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}
