use crate::Error;

use http_body_util::{BodyExt, Full};
use hyper::body::{Buf, Bytes, Incoming};
use hyper::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use hyper::{Request, Response, Uri};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use serde::Serialize;

use super::setup_tls::connector;

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
#[derive(Clone, Debug)]
pub struct RequestClient {
    client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
}

impl RequestClient {
    #[must_use]
    /// Creates a client to be passed to the LCU and in game structs
    pub fn new() -> Self {
        // Get a client config using the riotgames.pem file
        let tls = connector();
        // Set up an HTTPS only client, with just the client config
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls.clone())
            .https_only()
            .enable_http1()
            .build();
        // Make the new client
        let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);

        Self { client }
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
        T: Serialize + Send,
    {
        const MINE: &str = "application/x-msgpack";

        // Build the URI, always in https format
        let built_uri = Uri::builder()
            .scheme("https")
            .authority(url)
            .path_and_query(endpoint)
            .build()?;

        // Build the new request
        let mut builder = Request::builder()
            .method(method)
            .uri(built_uri)
            .header(CONTENT_TYPE, MINE)
            .header(ACCEPT, MINE);

        // Add the auth header, if provided
        if let Some(header) = auth_header {
            builder = builder.header(AUTHORIZATION, header);
        };

        let mut buffer = Full::default();

        // Turn the body to bytes, return any errors,
        // then map to Full<Bytes>
        if let Some(body) = body {
            buffer = Full::from(rmp_serde::to_vec_named(&body)?);
        }

        // Add the body to finalize
        let request = builder.body(buffer)?;

        // Return the incoming request
        Ok(self.client.request(request).await?)
    }

    /// Makes a request, collects the bytes, and returns the buf
    pub(crate) async fn request_template<T>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
    ) -> Result<impl Buf + Sized, Error>
    where
        T: Serialize + Send,
    {
        let response = self
            .raw_request_template(url, endpoint, method, body, auth_header)
            .await?;

        let body = response.collect().await?;

        Ok(body.aggregate())
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}
