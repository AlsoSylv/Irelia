use crate::LCUError;

use crate::RequestClient;
use hyper::body::Bytes;
use hyper::header::AUTHORIZATION;
use hyper::http::uri;
use hyper::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::setup_tls::setup_tls_connector;

impl RequestClient {
    /// Creates a client to be passed to the LCU and in game structs
    pub fn new() -> RequestClient {
        let tls = setup_tls_connector();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_or_http()
            .enable_http1()
            .build();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);

        RequestClient { client }
    }

    pub(crate) async fn request_template<T, R>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
        return_logic: fn(bytes: Bytes) -> Result<R, LCUError>,
    ) -> Result<R, LCUError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let uri = uri::Builder::new()
            .scheme("https")
            .authority(url.as_bytes())
            .path_and_query(endpoint)
            .build()
            .map_or_else(|err| Err(LCUError::HyperHttpError(err)), Ok)?;

        let body = match body {
            Some(body) => match serde_json::value::to_value(body) {
                Ok(json) => Ok(hyper::Body::from(json.to_string())),
                Err(err) => Err(LCUError::SerdeJsonError(err)),
            },
            None => Ok(hyper::Body::empty()),
        }?;

        let req = match auth_header {
            Some(header) => Request::builder()
                .method(method)
                .uri(uri)
                .header(AUTHORIZATION, header)
                .body(body),
            None => Request::builder().method(method).uri(uri).body(body),
        }
        .map_err(LCUError::HyperHttpError)?;

        let mut res = self
            .client
            .request(req)
            .await
            .map_err(LCUError::HyperError)?;
        
        let body = res.body_mut();

        match hyper::body::to_bytes(body).await {
            Ok(bytes) => return_logic(bytes),
            Err(err) => panic!("{}", err),
        }
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}
