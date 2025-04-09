use std::io::Write;
use std::net::SocketAddrV4;

use crate::Error;
use crate::error::HttpError;

use http::HeaderValue;
use serde::{Deserialize, Serialize};

const LONGEST_SOCKET_ADDR: usize = "255.255.255.255:65535".len();

pub trait RequestClientTrait: Sync {
    type Response: Send;

    type ResponseBytes: Send;

    type Error: Send + HttpError;

    fn raw_request_template(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<Vec<u8>>,
        auth_header: Option<&HeaderValue>,
        format: RequestFmt,
    ) -> impl std::future::Future<Output = Result<Self::Response, Error<Self::Error>>> + Send;

    fn request_template(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<Vec<u8>>,
        auth_header: Option<&HeaderValue>,
        format: RequestFmt,
    ) -> impl std::future::Future<Output = Result<Self::ResponseBytes, Error<Self::Error>>> + Send;

    /// Decodes the bytes from a request, which are in the specified format
    ///
    /// # Errors
    /// The decoder returned an error of some sort
    fn decode_response_bytes<R: for<'a> Deserialize<'a>>(
        bytes: Self::ResponseBytes,
        format: RequestFmt,
    ) -> Result<R, crate::error::SerdeError>;

    fn socketv4_raw_request_template(
        &self,
        url: SocketAddrV4,
        endpoint: &str,
        method: &str,
        body: Option<Vec<u8>>,
        auth_header: Option<&HeaderValue>,
        format: RequestFmt,
    ) -> impl std::future::Future<Output = Result<Self::Response, Error<Self::Error>>> + Send {
        async move {
            let mut buffer = [0; LONGEST_SOCKET_ADDR];
            let url = Self::socket_addr_to_str(&mut buffer, url);

            self.raw_request_template(url, endpoint, method, body, auth_header, format)
                .await
        }
    }

    fn socketv4_request_template(
        &self,
        url: SocketAddrV4,
        endpoint: &str,
        method: &str,
        body: Option<Vec<u8>>,
        auth_header: Option<&HeaderValue>,
        format: RequestFmt,
    ) -> impl std::future::Future<Output = Result<Self::ResponseBytes, Error<Self::Error>>> + Send
    {
        async move {
            let mut buffer = [0; LONGEST_SOCKET_ADDR];
            let url = Self::socket_addr_to_str(&mut buffer, url);

            self.request_template(url, endpoint, method, body, auth_header, format)
                .await
        }
    }

    fn socket_addr_to_str(buffer: &mut [u8; 21], url: SocketAddrV4) -> &str {
        let _ = write!(buffer.as_mut_slice(), "{url}");

        std::str::from_utf8(buffer).unwrap()
    }

    /// Encodes the request into a specified format
    ///
    /// # Errors
    /// The decoder returned an error of some sort
    fn encode_body<T: Serialize + Send>(
        body: Option<T>,
        format: RequestFmt,
    ) -> Result<Option<Vec<u8>>, crate::error::SerdeError> {
        if let Some(body) = body {
            return match format {
                RequestFmt::Json => todo!(),
                RequestFmt::MsgPack => rmp_serde::to_vec_named(&body),
            }
            .map(Option::Some)
            .map_err(From::from);
        }

        Ok(None)
    }
}

#[cfg(all(feature = "__hyper", not(feature = "__reqwest")))]
pub use hyper_client::*;

#[cfg(all(feature = "__hyper", not(feature = "__reqwest")))]
mod hyper_client {
    use std::fmt::Display;
    use std::future::Future;
    use std::pin::Pin;

    use bytes::Bytes;
    use http::HeaderValue;
    use http::uri::Scheme;
    use http_body_util::{BodyExt, Collected, Full};
    use hyper::body::Incoming;
    use hyper::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
    use hyper::rt::Executor;
    use hyper::{Request, Response, Uri};
    use hyper_util::client::legacy::Client;
    use hyper_util::rt::TokioExecutor;
    use serde::Deserialize;

    use crate::Error;
    use crate::error::HttpError;

    use super::{RequestClientTrait, RequestFmt};

    type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

    pub type RequestClientType = Client<crate::tls::Connector, Full<Bytes>>;

    #[must_use]
    pub fn new() -> RequestClientType {
        new_with_executor(TokioExecutor::new())
    }

    #[must_use]
    pub fn new_with_executor<E: Executor<BoxFuture> + Clone + Send + Sync + 'static>(
        exec: E,
    ) -> RequestClientType {
        let https = crate::tls::https_connector();

        Client::builder(exec).build(https)
    }

    #[non_exhaustive]
    #[derive(Debug)]
    pub enum HyperError {
        /// http error, re-exported by hyper
        HttpError(hyper::http::Error),
        /// Client error from `hyper_util`
        ClientError(hyper_util::client::legacy::Error),
        /// Hyper error
        Error(hyper::Error),
    }

    impl Display for HyperError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                HyperError::HttpError(error) => std::fmt::Display::fmt(error, f),
                HyperError::ClientError(error) => std::fmt::Display::fmt(error, f),
                HyperError::Error(error) => std::fmt::Display::fmt(error, f),
            }
        }
    }

    impl From<hyper::http::Error> for Error<HyperError> {
        fn from(value: hyper::http::Error) -> Self {
            HyperError::HttpError(value).into()
        }
    }

    impl From<hyper_util::client::legacy::Error> for Error<HyperError> {
        fn from(value: hyper_util::client::legacy::Error) -> Self {
            HyperError::ClientError(value).into()
        }
    }

    impl From<hyper::Error> for Error<HyperError> {
        fn from(value: hyper::Error) -> Self {
            HyperError::Error(value).into()
        }
    }

    impl std::error::Error for HyperError {}

    impl HttpError for HyperError {
        fn invalid_header_value(invalid_header_value: hyper::header::InvalidHeaderValue) -> Self {
            Self::HttpError(hyper::http::Error::from(invalid_header_value))
        }
    }

    impl RequestClientTrait for RequestClientType {
        type Response = Response<Incoming>;

        type ResponseBytes = Collected<Bytes>;

        type Error = HyperError;

        async fn raw_request_template(
            &self,
            url: &str,
            endpoint: &str,
            method: &str,
            body: Option<Vec<u8>>,
            auth_header: Option<&HeaderValue>,
            format: RequestFmt,
        ) -> Result<Self::Response, Error<Self::Error>> {
            // Build the URI, always in https format
            let built_uri = Uri::builder()
                .scheme(Scheme::HTTPS)
                .authority(url)
                .path_and_query(endpoint)
                .build()
                .unwrap();

            // Build the new request
            let mut builder = Request::builder()
                .method(method)
                .uri(built_uri)
                .header(CONTENT_TYPE, format.mime())
                .header(ACCEPT, format.mime());

            // Add the auth header, if provided
            if let Some(header) = auth_header {
                builder = builder.header(AUTHORIZATION, header);
            }

            let body = body.unwrap_or_default();

            // Add the body to finalize
            let request = builder.body(Full::from(body)).unwrap();
            // Return the incoming request
            Ok(self.request(request).await.unwrap())
        }

        async fn request_template(
            &self,
            url: &str,
            endpoint: &str,
            method: &str,
            body: Option<Vec<u8>>,
            auth_header: Option<&HeaderValue>,
            format: RequestFmt,
        ) -> Result<Self::ResponseBytes, Error<Self::Error>> {
            let response = self
                .raw_request_template(url, endpoint, method, body, auth_header, format)
                .await?;

            if !response.status().is_success() {
                return Err(Error::RequestError(response.status()));
            }

            let body = response.collect().await?;

            Ok(body)
        }

        fn decode_response_bytes<R: for<'a> Deserialize<'a>>(
            bytes: Self::ResponseBytes,
            format: RequestFmt,
        ) -> Result<R, crate::error::SerdeError> {
            use bytes::Buf;

            let reader = bytes.aggregate().reader();

            Ok(match format {
                RequestFmt::Json => serde_json::from_reader(reader)?,
                RequestFmt::MsgPack => rmp_serde::decode::from_read(reader)?,
            })
        }
    }
}

#[cfg(all(feature = "__reqwest", not(feature = "__hyper")))]
pub use reqwest_client::*;

#[cfg(all(feature = "__reqwest", not(feature = "__hyper")))]
mod reqwest_client {
    use std::fmt::Display;

    use bytes::Bytes;
    use http::{
        HeaderValue,
        header::{ACCEPT, CONTENT_TYPE},
    };
    use reqwest::Client;
    use serde::Deserialize;

    use crate::{Error, error::SerdeError};

    use super::{RequestClientTrait, RequestFmt};

    pub type RequestClientType = Client;

    /// # Panics
    /// panics if the cert is invalid (this should never happen)
    #[must_use]
    pub fn new() -> RequestClientType {
        let cert = crate::tls::connector_internal();

        RequestClientType::builder()
            .tls_built_in_root_certs(false)
            .use_preconfigured_tls(cert)
            .build()
            .unwrap()
    }

    #[non_exhaustive]
    #[derive(Debug)]
    pub enum ReqwestError {
        Error(reqwest::Error),
        InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
    }

    impl Display for ReqwestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ReqwestError::Error(error) => std::fmt::Display::fmt(error, f),
                ReqwestError::InvalidHeaderValue(invalid_header_value) => {
                    f.write_str(&invalid_header_value.to_string())
                }
            }
        }
    }

    impl std::error::Error for ReqwestError {}
    impl crate::error::HttpError for ReqwestError {
        fn invalid_header_value(invalid_header_value: http::header::InvalidHeaderValue) -> Self {
            Self::InvalidHeaderValue(invalid_header_value)
        }
    }

    impl From<reqwest::Error> for Error<ReqwestError> {
        fn from(value: reqwest::Error) -> Self {
            Self::HttpError(ReqwestError::Error(value))
        }
    }

    impl RequestClientTrait for reqwest::Client {
        type Response = reqwest::Response;

        type ResponseBytes = Bytes;

        type Error = ReqwestError;

        async fn raw_request_template(
            &self,
            url: &str,
            endpoint: &str,
            method: &str,
            body: Option<Vec<u8>>,
            auth_header: Option<&HeaderValue>,
            format: RequestFmt,
        ) -> Result<Self::Response, Error<Self::Error>> {
            let method = reqwest::Method::from_bytes(method.as_bytes()).unwrap();
            let url = format!("{url}{endpoint}");
            let mut request = self
                .request(method, url)
                .header(CONTENT_TYPE, format.mime())
                .header(ACCEPT, format.mime());

            if let Some(body) = body {
                request = request.body(body);
            }

            if let Some(auth_header) = auth_header {
                request = request.header(reqwest::header::AUTHORIZATION, auth_header);
            }

            Ok(request.send().await?)
        }

        async fn request_template(
            &self,
            url: &str,
            endpoint: &str,
            method: &str,
            body: Option<Vec<u8>>,
            auth_header: Option<&HeaderValue>,
            format: RequestFmt,
        ) -> Result<Self::ResponseBytes, Error<Self::Error>> {
            let response = self
                .raw_request_template(url, endpoint, method, body, auth_header, format)
                .await?;

            Ok(response.bytes().await?)
        }

        fn decode_response_bytes<R: for<'a> Deserialize<'a>>(
            bytes: Self::ResponseBytes,
            format: RequestFmt,
        ) -> Result<R, SerdeError> {
            Ok(match format {
                RequestFmt::Json => serde_json::from_slice(&bytes)?,
                RequestFmt::MsgPack => rmp_serde::from_slice(&bytes)?,
            })
        }
    }
}

#[cfg(all(feature = "__hyper", feature = "__reqwest"))]
compile_error!("Cannot use hyper and reqwest at the same time as a backend!");

pub enum RequestFmt {
    Json,
    MsgPack,
}

impl RequestFmt {
    #[must_use]
    pub fn mime(&self) -> &'static str {
        match self {
            RequestFmt::Json => todo!(),
            RequestFmt::MsgPack => todo!(),
        }
    }
}
