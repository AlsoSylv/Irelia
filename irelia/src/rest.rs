//! Module containing all the data for the rest LCU bindings

/// This is a list of types pertaining to the LCU, currently only containing the types for the schema.
pub mod types;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;
use std::net::SocketAddr;

use crate::rest::request_builder::RequestBuilder;
use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::utils::requests::SerializeFormat;
use crate::{utils::process_info::get_running_client, Error, RequestClient};

/// Struct representing a connection to the LCU
pub struct LcuClient {
    url: String,
    auth_header: String,
}

/// Some LCU endpoints don't return a response at all. This is used to convert an Eof row: 0, colum: 0 into an optional
pub trait EofIntoOptional {
    /// Returns the result as `Option<T>`, converting `Err(serde_json::Eof)` into `Ok(None)` instead
    /// Only when line and column equal 0
    ///
    /// # Errors
    /// If the result was an error before, and it was not an Eof Error
    fn eof_into_optional<T>(result: Result<T, Error>) -> Result<Option<T>, Error> {
        let result = result.map(Some);

        if let Err(Error::SerdeJsonError(err)) = &result {
            if err.is_eof() && err.line() == 0 && err.column() == 0 {
                return Ok(None);
            }
        }

        result
    }
}

impl<T> EofIntoOptional for Result<T, Error> {}

impl LcuClient {
    /// Attempts to create a connection to the LCU, errors if it fails
    /// to spin up the child process, or fails to get data from the client.
    ///
    /// `force_lock_file` will read the lock file regardless of whether the client
    /// or the game is running at the time
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, this can include
    /// the client being down, the lock file being unable to be opened, or the LCU
    /// not running at all
    pub fn new(force_lock_file: bool) -> Result<Self, Error> {
        let (addr, pass) =
            get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, force_lock_file)?;

        Ok(Self::new_with_credentials(addr, pass))
    }

    #[must_use]
    /// Creates a new LCU Client that implicitly trusts the port and auth string given,
    /// Encoding them in a URL and header respectively
    pub fn new_with_credentials(url: SocketAddr, auth_header: String) -> Self {
        Self { url: url.to_string(), auth_header }
    }

    /// Queries the client or lock file, getting a new url and auth header
    ///
    /// # Errors
    /// This will return an error if the lock file is inaccessible, or if
    /// the LCU is not running
    pub fn reconnect(&mut self, force_lock_file: bool) -> Result<(), Error> {
        let (addr, pass) =
            get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, force_lock_file)?;
        self.reconnect_with_credentials(addr, pass);
        Ok(())
    }

    /// Sets the url and auth header according to the auth and port provided
    pub fn reconnect_with_credentials(&mut self, url: SocketAddr, auth: String) {
        self.url = url.to_string();
        self.auth_header = auth;
    }

    #[must_use]
    /// Returns a reference to the URL in use
    pub fn url(&self) -> &str {
        &self.url
    }

    #[must_use]
    /// Returns a reference to the auth header in use
    pub fn auth_header(&self) -> &str {
        &self.auth_header
    }

    /// Sends a delete request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type is invalid
    pub async fn delete<R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        self.lcu_request(endpoint.as_ref(), "DELETE", None::<()>, request_client)
            .await
    }

    /// Sends a get request to the LCU
    /// ```
    /// let request_client = irelia::RequestClient::new();
    /// let lcu_client = irelia::rest::LcuClient::new(false)?;
    ///
    /// let response: Option<serde_json::Value> = lcu_client.get("/example/endpoint/", &request_client)?;
    /// ```
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type is invalid
    pub async fn get<R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        self.lcu_request(endpoint.as_ref(), "GET", None::<()>, request_client)
            .await
    }

    /// Sends a head request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running
    pub async fn head(
        &self,
        endpoint: impl AsRef<str> + Send,
        request_client: &RequestClient,
    ) -> Result<hyper::Response<hyper::body::Incoming>, Error> {
        request_client
            .raw_request_template(
                &self.url,
                endpoint.as_ref(),
                "HEAD",
                None::<()>,
                Some(&self.auth_header),
                SerializeFormat::Json,
            )
            .await
    }

    /// Sends a patch request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn patch<T: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        body: T,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        self.lcu_request(endpoint.as_ref(), "PATCH", Some(body), request_client)
            .await
    }

    /// Sends a post request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn post<T: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        body: T,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        self.lcu_request(endpoint.as_ref(), "POST", Some(body), request_client)
            .await
    }

    /// Sends a put request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn put<T: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        body: T,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        self.lcu_request(endpoint.as_ref(), "PUT", Some(body), request_client)
            .await
    }

    /// Makes a `Request` to the LCU client, using the details entered
    ///
    /// # Errors
    /// This will return an error if:
    /// - The body is not valid JSON
    /// - If the provided return type is invalid
    /// - It is unable to connect to the LCU
    /// - The LCU does not have the endpoint specified  
    pub async fn request<T: Serialize + Send, R: DeserializeOwned>(
        &self,
        request: Request<'_, T>,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        self.lcu_request(
            request.endpoint.as_ref(),
            request.method.as_str(),
            request.body,
            request_client,
        )
        .await
    }

    /// Makes a request to the LCU with an unspecified method, valid options being
    /// "PUT", "GET", "POST", "HEAD", "DELETE"
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    ///
    /// If the response body is empty, this will return an unexpected EOF error
    pub async fn lcu_request<T: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        request_client: &RequestClient,
    ) -> Result<R, Error> {
        use hyper::body::Buf;

        let buf = request_client
            .request_template(
                &self.url,
                endpoint,
                method,
                body,
                Some(&self.auth_header),
                SerializeFormat::Json,
            )
            .await?;

        Ok(serde_json::from_reader(buf.reader())?)
    }
}

mod request_builder {
    use crate::rest::Method;
    use serde::Serialize;
    use std::borrow::Cow;

    pub struct RequestBuilder<'a, T> {
        pub(super) method: Option<Method>,
        pub(super) endpoint: Option<Cow<'a, str>>,
        pub(super) body: Option<T>,
    }

    impl<'a, T: Serialize> RequestBuilder<'a, T> {
        pub fn endpoint(mut self, endpoint: impl Into<Cow<'a, str>>) -> Self {
            self.endpoint = Some(endpoint.into());

            self
        }

        pub fn body(mut self, body: T) -> Self {
            self.body = Some(body);

            self
        }

        pub const fn method(mut self, method: Method) -> Self {
            self.method = Some(method);

            self
        }

        pub fn build(self) -> super::Request<'a, T> {
            super::Request {
                method: self.method.expect("Must enter a valid method"),
                body: self.body,
                endpoint: self.endpoint.expect("Must enter a valid endpoint here"),
            }
        }
    }
}

#[doc(hidden)]
pub enum Method {
    Delete,
    Get,
    Head,
    Patch,
    Post,
    Put,
}

impl Method {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Patch => "PATCH",
            Self::Post => "POST",
            Self::Put => "PUT",
        }
    }
}

#[doc(hidden)]
pub struct Request<'a, T> {
    method: Method,
    endpoint: Cow<'a, str>,
    body: Option<T>,
}

impl<'a> Request<'a, ()> {
    #[must_use]
    pub fn new(endpoint: impl Into<Cow<'a, str>>, method: Method) -> Self {
        Self {
            method,
            body: None::<()>,
            endpoint: endpoint.into(),
        }
    }
}

impl<'a, T: Serialize> Request<'a, T> {
    #[must_use]
    pub const fn builder() -> RequestBuilder<'a, T> {
        RequestBuilder {
            method: None,
            endpoint: None,
            body: None,
        }
    }
}

/// Fetches the schema from a remote endpoint, for example:
/// <`https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json/`>
///
/// This return `None` if the http client could not be build
///
/// # Errors
///
/// This function will error if it fails to connect to the given remote,
/// or if the given remote cannot be deserialized to match the `Schema` type
///
/// # Panics
/// This panics if no valid
pub async fn schema(remote: &'static str) -> Result<Option<types::Schema>, Error> {
    use http_body_util::BodyExt;

    let uri = hyper::Uri::from_static(remote);
    // This creates a custom client, as the default hyper client used by Irelia needs a cert, and it has no use outside here
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .ok();

    let https = if let Some(https) = https {
        https.https_only().enable_http1().build()
    } else {
        return Ok(None);
    };

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build::<_, http_body_util::Full<hyper::body::Bytes>>(https);
    let mut request = client.get(uri).await?;
    let tmp = request.body_mut();
    let body = tmp.collect().await?.to_bytes();
    Ok(Some(serde_json::from_slice(&body)?))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_schema_des() {
        let _schema = super::schema(
            "https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json",
        )
        .await
        .unwrap();
    }
}
