//! Module containing all the data for the rest LCU bindings
//!
//! For responses that have no body, use `IgnoreAny` instead of supplying a type, or using an `Option<T>`

#[cfg(feature = "rest_schema")]
/// This is a list of types pertaining to the LCU, currently only containing the types for the schema.
pub mod types;

use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::{utils::process_info::get_running_client, Error, RequestClient};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::net::SocketAddrV4;

/// Struct representing a connection to the LCU
pub struct LcuClient {
    url: SocketAddrV4,
    auth_header: String,
}

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
    pub fn new_with_credentials(url: SocketAddrV4, auth_header: String) -> Self {
        Self { url, auth_header }
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
    pub fn reconnect_with_credentials(&mut self, url: SocketAddrV4, auth: String) {
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
                self.url,
                endpoint.as_ref(),
                "HEAD",
                None::<()>,
                Some(&self.auth_header),
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
            .request_template(self.url, endpoint, method, body, Some(&self.auth_header))
            .await?;

        Ok(rmp_serde::from_read(buf.reader())?)
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
