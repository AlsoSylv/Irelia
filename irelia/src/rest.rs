//! Module containing all the data for the rest LCU bindings

pub mod types;

use http_body_util::BodyExt;
use hyper::Uri;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;

use crate::rest::request_builder::RequestBuilder;
use crate::utils::process_info::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
use crate::{utils::process_info::get_running_client, Error, RequestClient};

/// Struct representing a connection to the LCU
pub struct LcuClient {
    url: String,
    auth_header: String,
}

#[cfg(feature = "batched")]
pub mod batch {
    use crate::rest::LcuClient;
    use crate::{Error, RequestClient};
    use futures_util::StreamExt;
    use serde::de::DeserializeOwned;
    use std::borrow::Cow;

    /// Enum representing the different requests that can be sent to the LCU
    pub enum RequestType<'a> {
        Delete,
        Get,
        Patch(Option<&'a dyn erased_serde::Serialize>),
        Post(Option<&'a dyn erased_serde::Serialize>),
        Put(Option<&'a dyn erased_serde::Serialize>),
    }

    /// Struct representing a batched request, taking the
    /// request type and endpoint
    pub struct Request<'a> {
        pub request_type: RequestType<'a>,
        pub endpoint: Cow<'static, str>,
    }

    impl<'a> Request<'a> {
        /// Creates a new batched request, which can be wrapped in a slice and send to the LCU
        pub fn new(request_type: RequestType<'a>, endpoint: impl Into<Cow<'static, str>>) -> Self {
            Request {
                request_type,
                endpoint: endpoint.into(),
            }
        }

        pub fn delete(endpoint: impl Into<Cow<'static, str>>) -> Self {
            Self::new(RequestType::Delete, endpoint)
        }

        pub fn get(endpoint: impl Into<Cow<'static, str>>) -> Self {
            Self::new(RequestType::Get, endpoint)
        }

        pub fn patch(
            endpoint: impl Into<Cow<'static, str>>,
            body: Option<&'a dyn erased_serde::Serialize>,
        ) -> Self {
            Self::new(RequestType::Patch(body), endpoint)
        }

        pub fn put(
            endpoint: impl Into<Cow<'static, str>>,
            body: Option<&'a dyn erased_serde::Serialize>,
        ) -> Self {
            Self::new(RequestType::Put(body), endpoint)
        }

        pub fn post(
            endpoint: impl Into<Cow<'static, str>>,
            body: Option<&'a dyn erased_serde::Serialize>,
        ) -> Self {
            Self::new(RequestType::Post(body), endpoint)
        }
    }

    impl LcuClient {
        /// System for batching requests to the LCU by sending a slice
        /// The buffer size is how many requests can be operated on at
        /// the same time, returns a vector with all the replies
        ///
        /// # Errors
        /// The value will be an error if the provided type is invalid, or the LCU API is not running
        pub async fn batched<'a, R>(
            &self,
            requests: &[Request<'a>],
            buffer_size: usize,
            request_client: &RequestClient,
        ) -> Vec<Result<Option<R>, Error>>
        where
            R: DeserializeOwned,
        {
            futures_util::stream::iter(requests.iter().map(|request| async {
                let endpoint = &*request.endpoint;
                match &request.request_type {
                    RequestType::Delete => self.delete(endpoint, request_client).await,
                    RequestType::Get => self.get(endpoint, request_client).await,
                    RequestType::Patch(body) => self.patch(endpoint, *body, request_client).await,
                    RequestType::Post(body) => self.post(endpoint, *body, request_client).await,
                    RequestType::Put(body) => self.put(endpoint, *body, request_client).await,
                }
            }))
            .buffered(buffer_size)
            .collect()
            .await
        }
    }

    pub struct Builder;

    mod hidden {
        use crate::rest::batch::Request;
        use crate::rest::LcuClient;
        use crate::RequestClient;

        pub struct WithClient<'a> {
            pub(super) request_client: &'a RequestClient,
            pub(super) requests: Vec<Request<'a>>,
        }

        pub struct WithBufferSize<'a> {
            pub(super) request_client: &'a RequestClient,
            pub(super) requests: Vec<Request<'a>>,
            pub(super) buffer_size: usize,
        }

        pub struct WithLcuClient<'a> {
            pub(super) request_client: &'a RequestClient,
            pub(super) requests: Vec<Request<'a>>,
            pub(super) buffer_size: usize,
            pub(super) lcu_client: &'a LcuClient,
        }
    }

    use crate::rest::batch::hidden::WithLcuClient;
    use hidden::{WithBufferSize, WithClient};

    impl Builder {
        #[must_use]
        pub fn new() -> Self {
            Self
        }

        #[must_use]
        pub fn with_client(self, request_client: &RequestClient) -> WithClient {
            WithClient {
                request_client,
                requests: Vec::new(),
            }
        }

        #[must_use]
        pub fn with_client_and_capacity(
            self,
            request_client: &RequestClient,
            capacity: usize,
        ) -> WithClient {
            WithClient {
                request_client,
                requests: Vec::with_capacity(capacity),
            }
        }
    }

    impl Default for Builder {
        fn default() -> Self {
            Self
        }
    }

    impl<'a> WithClient<'a> {
        pub fn request(mut self, request: Request<'a>) -> Self {
            self.add_request(request);

            self
        }

        pub fn add_request(&mut self, request: Request<'a>) {
            self.requests.push(request);
        }

        pub fn with_buffer_size(self, buffer_size: usize) -> WithBufferSize<'a> {
            WithBufferSize {
                requests: self.requests,
                request_client: self.request_client,
                buffer_size,
            }
        }
    }

    impl<'a> WithBufferSize<'a> {
        pub fn with_lcu_client(self, lcu_client: &'a LcuClient) -> WithLcuClient<'a> {
            WithLcuClient {
                requests: self.requests,
                request_client: self.request_client,
                buffer_size: self.buffer_size,
                lcu_client,
            }
        }
    }

    impl<'a> WithLcuClient<'a> {
        pub async fn execute<R: DeserializeOwned>(self) -> Vec<Result<Option<R>, Error>> {
            self.lcu_client
                .batched(&self.requests, self.buffer_size, self.request_client)
                .await
        }
    }
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
        let (port, pass) =
            get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, force_lock_file)?;

        Ok(LcuClient {
            url: port,
            auth_header: pass,
        })
    }

    #[must_use]
    /// Creates a new LCU Client that implicitly trusts the port and auth string given,
    /// Encoding them in a URL and header respectively
    pub fn new_with_credentials(auth: &str, port: u16) -> LcuClient {
        LcuClient {
            url: format!("127.0.0.1:{port}"),
            auth_header: format!(
                "Basic {}",
                crate::utils::process_info::ENCODER.encode(format!("riot:{auth}"))
            ),
        }
    }

    /// Queries the client or lock file, getting a new url and auth header
    ///
    /// # Errors
    /// This will return an error if the lock file is inaccessible, or if
    /// the LCU is not running
    pub fn reconnect(&mut self, force_lock_file: bool) -> Result<(), Error> {
        let (port, pass) =
            get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, force_lock_file)?;
        self.url = port;
        self.auth_header = pass;
        Ok(())
    }

    /// Sets the url and auth header according to the auth and port provided
    pub fn reconnect_with_credentials(&mut self, auth: &str, port: u16) {
        let port = format!("127.0.0.1:{port}");
        let pass = format!(
            "Basic {}",
            crate::utils::process_info::ENCODER.encode(format!("riot:{auth}"))
        );
        self.url = port;
        self.auth_header = pass;
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
        endpoint: impl AsRef<str>,
        request_client: &RequestClient,
    ) -> Result<Option<R>, Error> {
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
        endpoint: impl AsRef<str>,
        request_client: &RequestClient,
    ) -> Result<Option<R>, Error> {
        self.lcu_request(endpoint.as_ref(), "GET", None::<()>, request_client)
            .await
    }

    /// Sends a head request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running
    pub async fn head(
        &self,
        endpoint: impl AsRef<str>,
        request_client: &RequestClient,
    ) -> Result<hyper::Response<hyper::body::Incoming>, Error> {
        request_client
            .raw_request_template(
                &self.url,
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
    pub async fn patch<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str>,
        body: Option<T>,
        request_client: &RequestClient,
    ) -> Result<Option<R>, Error> {
        self.lcu_request(endpoint.as_ref(), "PATCH", Some(body), request_client)
            .await
    }

    /// Sends a post request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str>,
        body: Option<T>,
        request_client: &RequestClient,
    ) -> Result<Option<R>, Error> {
        self.lcu_request(endpoint.as_ref(), "POST", Some(body), request_client)
            .await
    }

    /// Sends a put request to the LCU
    ///
    /// # Errors
    /// This will return an error if the LCU API is not running, or the provided type or body is invalid
    pub async fn put<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: impl AsRef<str>,
        body: Option<T>,
        request_client: &RequestClient,
    ) -> Result<Option<R>, Error> {
        self.lcu_request(endpoint.as_ref(), "PUT", Some(body), request_client)
            .await
    }

    /// Fetches the schema from a remote endpoint, for example:
    /// <`https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json/`>
    ///
    /// # Errors
    ///
    /// This function will error if it fails to connect to the given remote,
    /// or if the given remote cannot be deserialized to match the `Schema` type
    pub async fn schema(remote: &'static str) -> Result<types::Schema, Error> {
        let uri = Uri::from_static(remote);
        // This creates a custom client, as the default hyper client used by Irelia needs a cert, and it has no use outside here
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .map_err(Error::StdIo)?
            .https_only()
            .enable_http1()
            .build();
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build::<_, http_body_util::Full<hyper::body::Bytes>>(https);
        let mut request = client.get(uri).await.map_err(Error::HyperClientError)?;
        let tmp = request.body_mut();
        let body = tmp.collect().await.map_err(Error::HyperError)?.to_bytes();
        serde_json::from_slice(&body).map_err(Error::SerdeJsonError)
    }

    /// Makes a `Request` to the LCU client, using the details entered
    ///
    /// # Errors
    /// This will return an error if:
    /// - The body is not valid JSON
    /// - If the provided return type is invalid
    /// - It is unable to connect to the LCU
    /// - The LCU does not have the endpoint specified  
    pub async fn request<T: Serialize, R: DeserializeOwned>(
        &self,
        request: Request<'_, T>,
        request_client: &RequestClient,
    ) -> Result<Option<R>, Error> {
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
    pub async fn lcu_request<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        request_client: &RequestClient,
    ) -> Result<Option<R>, Error> {
        use hyper::body::Buf;

        let buf = request_client
            .request_template(&self.url, endpoint, method, body, Some(&self.auth_header))
            .await?;

        let body = if buf.has_remaining() {
            Some(serde_json::from_reader(buf.reader())?)
        } else {
            None
        };

        Ok(body)
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

        pub fn method(mut self, method: Method) -> Self {
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
    pub fn as_str(&self) -> &str {
        match self {
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Patch => "PATCH",
            Method::Post => "POST",
            Method::Put => "PUT",
        }
    }
}

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
    pub fn builder() -> RequestBuilder<'a, T> {
        RequestBuilder {
            method: None,
            endpoint: None,
            body: None,
        }
    }
}

#[cfg(feature = "batched")]
#[cfg(test)]
mod tests {
    use crate::rest::Method;
    use crate::{rest::LcuClient, RequestClient};

    #[tokio::test]
    async fn batch_test() {
        use crate::rest::{
            batch::{Request, RequestType},
            LcuClient,
        };

        let page = serde_json::json!(
            {
              "blocks": [
                {
                  "items": [
                    {
                      "count": 1,
                      "id": "3153"
                    },
                  ],
                  "type": "Final Build"
                }
              ],
              "title": "Test Build",
            }
        );
        let client = RequestClient::new();

        let lcu_client = LcuClient::new(false).unwrap();

        let request: serde_json::Value = lcu_client
            .get("/lol-summoner/v1/current-summoner", &client)
            .await
            .unwrap()
            .unwrap();

        let id = &request["summonerId"];

        let endpoint = format!("/lol-item-sets/v1/item-sets/{id}/sets");

        let mut json: serde_json::Value = lcu_client
            .get(endpoint.as_str(), &client)
            .await
            .unwrap()
            .unwrap();

        json["itemSets"].as_array_mut().unwrap().push(page);

        let req = Request {
            request_type: RequestType::Put(Some(&json)),
            endpoint: format!("/lol-item-sets/v1/item-sets/{id}/sets").into(),
        };

        let result = lcu_client
            .batched::<serde_json::Value>(&[req], 1, &client)
            .await;

        println!("{result:?}");

        let request = super::Request::builder()
            .method(Method::Post)
            .endpoint(format!("/lol-item-sets/v1/item-sets/{id}/sets"))
            .body(&json)
            .build();

        let a = lcu_client
            .request::<_, serde_json::Value>(request, &client)
            .await;

        println!("{a:?}");
    }

    #[tokio::test]
    async fn test_schema_des() {
        let _schema = LcuClient::schema(
            "https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json",
        )
        .await
        .unwrap();
    }
}
