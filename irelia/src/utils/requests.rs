use crate::Error;

use http_body_util::{BodyExt, Full};
use hyper::body::{Buf, Bytes, Incoming};
use hyper::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use hyper::{Request, Response, Uri};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
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
#[derive(Clone, Debug)]
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
        format: SerializeFormat,
    ) -> Result<Response<Incoming>, Error>
    where
        T: Serialize,
    {
        // Build the URI, always in https format
        let built_uri = Uri::builder()
            .scheme("https")
            .authority(url.as_bytes())
            .path_and_query(endpoint)
            .build()?;

        // Build the new request
        let mut builder = Request::builder().method(method).uri(built_uri);

        // Add the auth header, if provided
        if let Some(header) = auth_header {
            builder = builder.header(AUTHORIZATION, header);
        };

        let mut buffer = Full::default();

        let mime = format.to_mime();

        builder = builder.header(CONTENT_TYPE, mime);
        builder = builder.header(ACCEPT, mime);

        // Turn the JSON to bytes, return any errors,
        // then map to Full<Bytes>
        if let Some(body) = body {
            let buf = if format.is_json() {
                serde_json::to_vec(&body)?
            } else {
                rmp_serde::to_vec(&body)?
            };

            buffer = Full::from(buf);
        }

        // Add the body to finalize
        let request = builder.body(buffer)?;

        // Return the incoming request
        Ok(self.client.request(request).await?)
    }

    /// Makes a request, collects the bytes, and passes them to `return_logic` for handling
    pub(crate) async fn request_template<T>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
        format: SerializeFormat
    ) -> Result<impl Buf + Sized, Error>
    where
        T: Serialize,
    {
        let response = self
            .raw_request_template(
                url,
                endpoint,
                method,
                body,
                auth_header,
                format,
            )
            .await?;

        let body = response.collect().await?;

        Ok(body.aggregate())
    }
}

// This isn't actually dead, just only when the replay API is not in use
#[allow(dead_code)]
#[repr(u8)]
/// The format to use for requests, currently only JSON or `MsgPack`
/// YAML support is possible if requested
pub(crate) enum SerializeFormat {
    Json,
    MsgPack,
}

impl SerializeFormat {
    fn to_mime(&self) -> &str {
        match &self {
            SerializeFormat::Json => "application/json",
            SerializeFormat::MsgPack => "application/msgpack",
        }
    }

    fn is_json(&self) -> bool {
        match &self {
            SerializeFormat::Json => true,
            SerializeFormat::MsgPack => false,
        }
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}
