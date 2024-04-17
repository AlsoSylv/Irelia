use crate::Error;

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
        // Get a client config using the riotgames.pem file
        let tls = setup_tls_connector();
        // Set up an HTTPS only client, with just the client config
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_only()
            .enable_http1()
            .build();
        // Make the new client
        let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);

        RequestClient { client }
    }

    /// returns a raw hyper response, URIs always use HTTPS,
    /// 
    /// # Errors
    /// if the body is invalid JSON, otherwise in any way hyper would normally
    pub(crate) async fn raw_request_template<T>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
    ) -> Result<Response<Incoming>, Error>
    where
        T: Serialize,
    {
        let built_uri = uri::Builder::new()
            .scheme("https")
            .authority(url.as_bytes())
            .path_and_query(endpoint)
            .build()?;

        let body = if let Some(body) = &body {
            let json = serde_json::value::to_value(body)?;
            Full::from(json.to_string())
        } else {
            Full::default()
        };

        let builder = Request::builder().method(method).uri(built_uri);

        let builder = if let Some(header) = auth_header {
            builder.header(AUTHORIZATION, header)
        } else {
            builder
        };

        let request = builder.body(body)?;

        Ok(self.client.request(request).await?)
    }

    /// Makes a request, collects the bytes, and passes them to `return_logic` for handling
    pub(crate) async fn request_template<T, R>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
        return_logic: fn(bytes: Bytes) -> Result<R, Error>,
    ) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let mut response = self
            .raw_request_template(url, endpoint, method, body, auth_header)
            .await?;

        let body = response.body_mut();

        let bytes = body.collect().await?.to_bytes();

        return_logic(bytes)
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}
