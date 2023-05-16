use crate::Error;

use hyper::body::Bytes;
use hyper::client::HttpConnector;
use hyper::header::AUTHORIZATION;
use hyper::http::uri;
use hyper::{Client, Request};
use hyper_rustls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::LazyLock;

use super::setup_tls::TLS_CONFIG;

#[cfg(any(feature = "in_game", feature = "rest"))]
/// Sets up a hyper client with a TLS connector and riots pem certificate
pub(crate) static HYPER_CLIENT: LazyLock<Client<HttpsConnector<HttpConnector>>> =
    LazyLock::new(|| {
        let tls = TLS_CONFIG.clone();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_or_http()
            .enable_http1()
            .build();
        hyper::Client::builder().build::<_, hyper::Body>(https)
    });

pub struct RequestClient<'a> {
    client: &'a LazyLock<hyper::Client<HttpsConnector<HttpConnector>>>,
}

impl RequestClient<'_> {
    pub fn new<'a>() -> RequestClient<'a> {
        RequestClient {
            client: &HYPER_CLIENT,
        }
    }

    pub(crate) async fn request_template<T, R>(
        &self,
        url: &str,
        endpoint: &str,
        method: &str,
        body: Option<T>,
        auth_header: Option<&str>,
        return_logic: fn(bytes: Bytes) -> Result<R, Error>,
    ) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let uri = uri::Builder::new()
            .scheme("https")
            .authority(url.as_bytes())
            .path_and_query(endpoint)
            .build()
            .map_or_else(|err| Err(Error::HyperHttpError(err)), Ok)?;

        let body = body.map_or_else(
            || Ok(hyper::Body::empty()),
            |body| {
                serde_json::value::to_value(body).map_or_else(
                    |err| Err(Error::SerdeJsonError(err)),
                    |json| Ok(hyper::Body::from(json.to_string())),
                )
            },
        )?;

        let req = match auth_header {
            Some(header) => Request::builder()
                .method(method)
                .uri(uri)
                .header(AUTHORIZATION, header)
                .body(body),
            None => Request::builder().method(method).uri(uri).body(body),
        }
        .map_err(Error::HyperHttpError)?;

        let mut res = self.client.request(req).await.map_err(Error::HyperError)?;
        let body = res.body_mut();
        match hyper::body::to_bytes(body).await {
            Ok(bytes) => return_logic(bytes),
            Err(err) => panic!("{}", err),
        }
    }
}
