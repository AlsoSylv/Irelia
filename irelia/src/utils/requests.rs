use crate::Error;
use std::fmt::Debug;
use std::future::Future;
use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddrV4;
use std::pin::Pin;

use http_body_util::{BodyExt, Collected, Full};
use hyper::body::{Bytes, Incoming};
use hyper::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use hyper::http::uri::Scheme;
use hyper::http::HeaderValue;
use hyper::rt::Executor;
use hyper::{Request, Response, Uri};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::Serialize;

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
    client: Client<crate::tls::Connector, Full<Bytes>>,
}

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

impl RequestClient {
    #[must_use]
    /// Creates a client to be passed to the LCU and in game structs
    pub fn new() -> Self {
        Self::new_with_executor(TokioExecutor::new())
    }

    #[must_use]
    pub fn new_with_executor<E: Executor<BoxFuture> + Clone + Send + Sync + 'static>(
        exec: E,
    ) -> Self {
        let https = crate::tls::https_connector();
        // Make the new client
        let client = Client::builder(exec).build(https);

        Self { client }
    }

    /// returns a raw hyper response, URIs always use HTTPS,
    ///
    /// # Errors
    /// if the body is invalid JSON, otherwise in any way hyper would normally
    pub(crate) async fn raw_request_template(
        &self,
        url: SocketAddrV4,
        endpoint: &str,
        method: &str,
        body: Option<Full<Bytes>>,
        auth_header: Option<&HeaderValue>,
    ) -> Result<Response<Incoming>, Error> {
        const MINE: &str = "application/x-msgpack";
        const LONGEST_SOCKET_ADDR: usize = "255.255.255.255:65535".len();

        let mut buffer = [0; LONGEST_SOCKET_ADDR];
        let mut buf_writer = BufWriter::new(buffer.as_mut_slice());

        // The socket addr is IpV4, so this is guaranteed to fit by the type system
        let _ = write!(&mut buf_writer, "{url}");

        // Build the URI, always in https format
        let built_uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(buf_writer.buffer())
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

        let body = body.unwrap_or_default();

        // Add the body to finalize
        let request = builder.body(body)?;

        // Return the incoming request
        Ok(self.client.request(request).await?)
    }

    /// Makes a request, collects the bytes, and returns the buf
    pub(crate) async fn request_template<T: Serialize + Send>(
        &self,
        url: SocketAddrV4,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&HeaderValue>,
    ) -> Result<Collected<Bytes>, Error> {
        let body = body
            .map(|body| rmp_serde::to_vec_named(&body).map(Full::from))
            .transpose()?;

        let response = self
            .raw_request_template(url, endpoint, method, body, auth_header)
            .await?;

        if !response.status().is_success() {
            return Err(Error::RequestError(response.status()));
        }

        let body = response.collect().await?;

        Ok(body)
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}
