//! Module containing all the data for the rest LCU bindings
//!
//! For responses that have no body, use `IgnoreAny` instead of supplying a type, or using an `Option<T>`

#[cfg(feature = "rest_schema")]
/// This is a list of types pertaining to the LCU, currently only containing the types for the schema.
pub mod types;

use crate::utils::process_info::get_running_client;
use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::utils::requests::RequestClientTrait;
use http::HeaderValue;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::net::SocketAddrV4;

/// Struct representing a connection to the LCU
pub struct LcuClient<T: RequestClientTrait> {
    request_client: T,
    url: SocketAddrV4,
    auth_header: HeaderValue,
}

impl<T: RequestClientTrait + Clone> Clone for LcuClient<T> {
    fn clone(&self) -> Self {
        Self {
            request_client: self.request_client.clone(),
            url: self.url.clone(),
            auth_header: self.auth_header.clone(),
        }
    }
}

#[cfg(any(feature = "__hyper", feature = "__reqwest"))]
impl LcuClient<crate::requests::RequestClientType> {
    /// Attempts to create a connection to the LCU, errors if it fails
    /// to spin up the child process, or fails to get data from the client.
    ///
    /// `request_client` will be the client used when creating the `LcuClient` struct
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, this can include
    /// the client being down, the lock file being unable to be opened, or the LCU
    /// not running at all
    pub fn connect()
    -> Result<Self, <crate::requests::RequestClientType as RequestClientTrait>::Error> {
        Self::connect_with_request_client_force_lockfile(false, &crate::requests::new())
    }
}

impl<T: RequestClientTrait + Clone> LcuClient<T> {
    /// Attempts to create a connection to the LCU, errors if it fails
    /// to spin up the child process, or fails to get data from the client.
    ///
    /// `request_client` will be the client used when creating the `LcuClient` struct
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, this can include
    /// the client being down, the lock file being unable to be opened, or the LCU
    /// not running at all
    pub fn connect_with_request_client(request_client: &T) -> Result<Self, T::Error> {
        Self::connect_with_request_client_force_lockfile(false, request_client)
    }

    /// Attempts to create a connection to the LCU, errors if it fails
    /// to spin up the child process, or fails to get data from the client.
    ///
    /// `force_lock_file` will read the lock file regardless of whether the client
    /// or the game is running at the time
    /// `request_client` will be the client used when creating the `LcuClient` struct
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, this can include
    /// the client being down, the lock file being unable to be opened, or the LCU
    /// not running at all
    pub fn connect_with_request_client_force_lockfile(
        force_lock_file: bool,
        request_client: &T,
    ) -> Result<Self, T::Error> {
        let (addr, pass) = get_running_client(
            CLIENT_PROCESS_NAME,
            GAME_PROCESS_NAME,
            force_lock_file,
            None,
        )?;

        Ok(Self::new_with_credentials_with_request_client(
            addr,
            pass?,
            request_client,
        ))
    }

    #[must_use]
    /// Creates a new LCU Client that implicitly trusts the port and auth string given,
    /// Encoding them in a URL and header respectively
    pub fn new_with_credentials_with_request_client(
        url: SocketAddrV4,
        auth_header: HeaderValue,
        request_client: &T,
    ) -> Self {
        Self {
            url,
            auth_header,
            request_client: request_client.clone(),
        }
    }

    /// Queries the client or lock file, getting a new url and auth header
    ///
    /// # Errors
    /// This will return an error if the lock file is inaccessible, or if
    /// the LCU is not running
    pub fn reconnect(&mut self, force_lock_file: bool) -> Result<(), T::Error> {
        let (addr, pass) = get_running_client(
            CLIENT_PROCESS_NAME,
            GAME_PROCESS_NAME,
            force_lock_file,
            None,
        )?;
        self.reconnect_with_credentials(addr, pass?);
        Ok(())
    }

    /// Sets the url and auth header according to the auth and port provided
    pub fn reconnect_with_credentials(&mut self, url: SocketAddrV4, auth: HeaderValue) {
        self.url = url;
        self.auth_header = auth;
    }

    #[must_use]
    /// Returns a reference to the URL in use
    pub fn url(&self) -> SocketAddrV4 {
        self.url
    }

    #[must_use]
    /// Returns a reference to the auth header in use
    pub fn auth_header(&self) -> &HeaderValue {
        &self.auth_header
    }

    #[must_use]
    /// Returns a reference to the request client being used, so other clients can be constructed from it
    pub fn request_client(&self) -> &T {
        &self.request_client
    }

    /// Sends a delete request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type is invalid
    pub async fn delete<R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
    ) -> Result<R, T::Error> {
        self.lcu_request(endpoint.as_ref(), "DELETE", None::<()>)
            .await
    }

    /// Sends a get request to the LCU
    /// ```
    /// let lcu_client = irelia::rest::LcuClient::connect()?;
    ///
    /// let response: Option<serde_json::Value> = lcu_client.get("/example/endpoint/")?;
    /// ```
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type is invalid
    pub async fn get<R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
    ) -> Result<R, T::Error> {
        self.lcu_request(endpoint.as_ref(), "GET", None::<()>).await
    }

    /// Sends a head request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running
    pub async fn head(&self, endpoint: impl AsRef<str> + Send) -> Result<T::Response, T::Error> {
        self.request_client
            .socketv4_raw_request_template(
                self.url,
                endpoint.as_ref(),
                "HEAD",
                None,
                Some(&self.auth_header),
                crate::requests::RequestFmt::MsgPack,
            )
            .await
    }

    /// Sends a patch request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn patch<B: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        body: B,
    ) -> Result<R, T::Error> {
        self.lcu_request(endpoint.as_ref(), "PATCH", Some(body))
            .await
    }

    /// Sends a post request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn post<B: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        body: B,
    ) -> Result<R, T::Error> {
        self.lcu_request(endpoint.as_ref(), "POST", Some(body))
            .await
    }

    /// Sends a put request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn put<B: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str> + Send,
        body: B,
    ) -> Result<R, T::Error> {
        self.lcu_request(endpoint.as_ref(), "PUT", Some(body)).await
    }

    /// Makes a request to the LCU with an unspecified method, valid options being
    /// "PUT", "GET", "POST", "HEAD", "DELETE"
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    ///
    /// If the response body is empty, this will return an unexpected EOF error
    pub async fn lcu_request<B: Serialize + Send, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<B>,
    ) -> Result<R, T::Error> {
        let buf = self
            .request_client
            .socketv4_request_template(
                self.url,
                endpoint,
                method,
                T::encode_body(body, crate::requests::RequestFmt::MsgPack)?,
                Some(&self.auth_header),
                crate::requests::RequestFmt::MsgPack,
            )
            .await?;

        Ok(T::decode_response_bytes(
            buf,
            crate::requests::RequestFmt::MsgPack,
        )?)
    }
}

#[cfg(feature = "rest_schema")]
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
    use hyper::body::Buf;

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
    let tmp = request.body_mut().collect().await?;
    Ok(serde_json::from_reader(tmp.aggregate().reader()).ok())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "rest_schema")]
    #[tokio::test]
    async fn test_schema_des() {
        let schema = super::schema(
            "https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json",
        )
        .await
        .unwrap();

        println!("{schema:?}");
    }
}
