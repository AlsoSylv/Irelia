use std::io::Write;
use std::net::SocketAddrV4;

use http::{HeaderValue, header::InvalidHeaderValue};
use serde::{Deserialize, Serialize};

const LONGEST_SOCKET_ADDR: usize = "255.255.255.255:65535".len();

/// This is an intermediary type when encoding/decoding bodies and resposnes
pub enum SerdeError {
    RmpEncode(rmp_serde::encode::Error),
    RmpDecode(rmp_serde::decode::Error),
    Json(serde_json::Error),
}

impl From<rmp_serde::encode::Error> for SerdeError {
    fn from(value: rmp_serde::encode::Error) -> Self {
        Self::RmpEncode(value)
    }
}

impl From<rmp_serde::decode::Error> for SerdeError {
    fn from(value: rmp_serde::decode::Error) -> Self {
        Self::RmpDecode(value)
    }
}

impl From<serde_json::Error> for SerdeError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

pub trait RequestClientTrait: Sync {
    type Response: Send;

    type ResponseBytes: Send;

    #[cfg(feature = "rest")]
    type Error: Send
        + From<SerdeError>
        + From<crate::process_info::Error>
        + From<InvalidHeaderValue>;

    #[cfg(not(feature = "rest"))]
    type Error: Send + From<SerdeError> + From<InvalidHeaderValue>;

    fn raw_request_template(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<Vec<u8>>,
        auth_header: Option<&HeaderValue>,
        format: RequestFmt,
    ) -> impl std::future::Future<Output = Result<Self::Response, Self::Error>> + Send;

    fn request_template(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<Vec<u8>>,
        auth_header: Option<&HeaderValue>,
        format: RequestFmt,
    ) -> impl std::future::Future<Output = Result<Self::ResponseBytes, Self::Error>> + Send;

    /// Decodes the bytes from a request, which are in the specified format
    ///
    /// # Errors
    /// The decoder returned an error of some sort
    fn decode_response_bytes<R: for<'a> Deserialize<'a>>(
        bytes: Self::ResponseBytes,
        format: RequestFmt,
    ) -> Result<R, SerdeError>;

    fn socketv4_raw_request_template(
        &self,
        url: SocketAddrV4,
        endpoint: &str,
        method: &str,
        body: Option<Vec<u8>>,
        auth_header: Option<&HeaderValue>,
        format: RequestFmt,
    ) -> impl std::future::Future<Output = Result<Self::Response, Self::Error>> + Send {
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
    ) -> impl std::future::Future<Output = Result<Self::ResponseBytes, Self::Error>> + Send {
        async move {
            let mut buffer = [0; LONGEST_SOCKET_ADDR];
            let url = Self::socket_addr_to_str(&mut buffer, url);

            self.request_template(url, endpoint, method, body, auth_header, format)
                .await
        }
    }

    fn socket_addr_to_str(buffer: &mut [u8; 21], url: SocketAddrV4) -> &str {
        let written = {
            let mut writer = std::io::BufWriter::new(buffer.as_mut_slice());
            let _ = write!(&mut writer, "{url}");
            writer.buffer().len()
        };

        std::str::from_utf8(&buffer[..written]).unwrap()
    }

    /// Encodes the request into a specified format
    ///
    /// # Errors
    /// The decoder returned an error of some sort
    fn encode_body<T: Serialize + Send>(
        body: Option<T>,
        format: RequestFmt,
    ) -> Result<Option<Vec<u8>>, SerdeError> {
        if let Some(body) = body {
            Ok(Some(match format {
                RequestFmt::Json => serde_json::to_vec(&body)?,
                RequestFmt::MsgPack => rmp_serde::to_vec_named(&body)?,
            }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(all(feature = "__hyper", not(feature = "__reqwest")))]
pub use hyper_client::*;

#[cfg(all(feature = "__hyper", not(feature = "__reqwest")))]
mod hyper_client {
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

    use super::create_error_type;

    create_error_type!(
        pub enum Error {
            #[doc = "Client error from `hyper_util`"]
            ClientError(hyper_util::client::legacy::Error),
            #[doc = "Hyper error"]
            Error(hyper::Error),
        }
    );

    impl RequestClientTrait for RequestClientType {
        type Response = Response<Incoming>;

        type ResponseBytes = Collected<Bytes>;

        type Error = Error;

        async fn raw_request_template(
            &self,
            url: &str,
            endpoint: &str,
            method: &str,
            body: Option<Vec<u8>>,
            auth_header: Option<&HeaderValue>,
            format: RequestFmt,
        ) -> Result<Self::Response, Self::Error> {
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
            Ok(self.request(request).await?)
        }

        async fn request_template(
            &self,
            url: &str,
            endpoint: &str,
            method: &str,
            body: Option<Vec<u8>>,
            auth_header: Option<&HeaderValue>,
            format: RequestFmt,
        ) -> Result<Self::ResponseBytes, Self::Error> {
            let response = self
                .raw_request_template(url, endpoint, method, body, auth_header, format)
                .await?;

            if !response.status().is_success() {
                return Err(Error::Request(response.status()));
            }

            let body = response.collect().await?;

            Ok(body)
        }

        fn decode_response_bytes<R: for<'a> Deserialize<'a>>(
            bytes: Self::ResponseBytes,
            format: RequestFmt,
        ) -> Result<R, super::SerdeError> {
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
    use bytes::Bytes;
    use http::{
        HeaderValue,
        header::{ACCEPT, CONTENT_TYPE},
    };
    use reqwest::Client;
    use serde::Deserialize;

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

    super::create_error_type!(
        pub enum Error {
            Reqwest(reqwest::Error),
        }
    );

    impl RequestClientTrait for reqwest::Client {
        type Response = reqwest::Response;

        type ResponseBytes = Bytes;

        type Error = Error;

        async fn raw_request_template(
            &self,
            url: &str,
            endpoint: &str,
            method: &str,
            body: Option<Vec<u8>>,
            auth_header: Option<&HeaderValue>,
            format: RequestFmt,
        ) -> Result<Self::Response, Self::Error> {
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
        ) -> Result<Self::ResponseBytes, Self::Error> {
            let response = self
                .raw_request_template(url, endpoint, method, body, auth_header, format)
                .await?;

            Ok(response.bytes().await?)
        }

        fn decode_response_bytes<R: for<'a> Deserialize<'a>>(
            bytes: Self::ResponseBytes,
            format: RequestFmt,
        ) -> Result<R, super::SerdeError> {
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
            RequestFmt::Json => "application/json",
            RequestFmt::MsgPack => "application/x-msgpack",
        }
    }
}

macro_rules! define_create_error_type {
    ([$_:tt] $(true $(@$cfg_feature_rest:tt)?)?) => {
       #[macro_export]
       macro_rules! create_error_type {
            (
                $_(#[$enum_meta:meta])*
                $vis:vis enum $enum_name:ident {
                    $_(
                        $_(#[$meta:meta])*
                        $name:ident($value:ty)
                    ),+ $_(,)?
                }
            ) => {
                $_(#[$enum_meta])*
                $vis enum $enum_name {
                    $_(
                        $_(#[$meta])*
                        $name($value),
                    )*
                    /// Error with the request, contains a status code
                    Request(http::StatusCode),
                    /// This should only be returned when using the rest API if the auth header is invalid
                    InvalidHeaderValue(http::header::InvalidHeaderValue),
                    /// This is only returned when the API is configured to use the MSGPack
                    RmpEncode(rmp_serde::encode::Error),
                    /// This is only returned when the API is configured to use the MSGPack
                    RmpDecode(rmp_serde::decode::Error),
                    /// This is only returned when the API is configured to use the JSON
                    Json(serde_json::Error),
                    $($($cfg_feature_rest)?
                        /// Error getting process info (only possible with the `rest` feature enabled)
                        ProcessInfo($crate::process_info::Error),
                    )?
                }

                $_(impl From<$value> for $enum_name {
                    fn from(value: $value) -> Self {
                        Self::$name(value)
                    }
                })*

                impl From<rmp_serde::encode::Error> for $enum_name {
                    fn from(value: rmp_serde::encode::Error) -> Self {
                        Self::RmpEncode(value)
                    }
                }

                impl From<rmp_serde::decode::Error> for $enum_name {
                    fn from(value: rmp_serde::decode::Error) -> Self {
                        Self::RmpDecode(value)
                    }
                }

                impl From<serde_json::Error> for $enum_name {
                    fn from(value: serde_json::Error) -> Self {
                        Self::Json(value)
                    }
                }

                impl From<http::header::InvalidHeaderValue> for $enum_name {
                    fn from(value: http::header::InvalidHeaderValue) -> Self {
                        Self::InvalidHeaderValue(value)
                    }
                }

                $($($cfg_feature_rest)?
                    impl From<$crate::process_info::Error> for $enum_name {
                        fn from(value: $crate::process_info::Error) -> Self {
                            Self::ProcessInfo(value)
                        }
                    }
                )?

                impl std::fmt::Display for $enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            $_(Self::$name(err) => err.fmt(f),)*
                            Self::InvalidHeaderValue(err) => err.fmt(f),
                            Self::Request(code) => f.write_str(code.as_str()),
                            $($($cfg_feature_rest)?
                                Self::ProcessInfo(err) => f.write_str(err.reason()),
                            )?
                            Self::RmpEncode(err) => err.fmt(f),
                            Self::RmpDecode(err) => err.fmt(f),
                            Self::Json(err) => err.fmt(f),
                        }
                    }
                }

                impl From<$crate::requests::SerdeError> for $enum_name {
                    fn from(value: $crate::requests::SerdeError) -> Self {
                        match value {
                            $crate::requests::SerdeError::RmpEncode(err) => Self::RmpEncode(err),
                            $crate::requests::SerdeError::RmpDecode(err) => Self::RmpDecode(err),
                            $crate::requests::SerdeError::Json(err) => Self::Json(err),
                        }
                    }
                }

                impl std::fmt::Debug for $enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            $_(Self::$name(err) => err.fmt(f),)*
                            Self::InvalidHeaderValue(err) => err.fmt(f),
                            Self::Request(code) => f.write_str(code.as_str()),
                            $($($cfg_feature_rest)?
                                Self::ProcessInfo(err) => f.write_str(err.reason()),
                            )?
                            Self::RmpEncode(err) => err.fmt(f),
                            Self::RmpDecode(err) => err.fmt(f),
                            Self::Json(err) => err.fmt(f),
                        }
                    }
                }

                impl std::error::Error for $enum_name {}

                impl serde::Serialize for $enum_name {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        serializer.serialize_str(&self.to_string())
                    }
                }
            };
       }

       pub(crate) use create_error_type;
    }
}

#[cfg(feature = "rest")]
define_create_error_type!([$] true);

#[cfg(not(feature = "rest"))]
define_create_error_type!([$]);
