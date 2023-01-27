use super::setup_tls::setup_tls_connector;
use hyper::body::Bytes;
use hyper::http::uri;
use hyper::{client::HttpConnector, Request};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;

use crate::Error;

#[cfg(any(feature = "in_game", feature = "rest"))]
/// Sets up a hyper client with a TLS connector and riots pem certificate
pub(crate) fn setup_hyper_client() -> Result<Client<HttpsConnector<HttpConnector>>, ()> {
    let tls = setup_tls_connector();
    let tokio_tls = tokio_native_tls::TlsConnector::from(tls);
    let mut http = HttpConnector::new();
    http.enforce_http(false);
    let https = HttpsConnector::from((http, tokio_tls));
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    Ok(client)
}

#[cfg(any(feature = "in_game", feature = "rest"))]
pub(crate) async fn request_template<Return: DeserializeOwned>(
    running_error: Error,
    request: Result<Request<hyper::Body>, hyper::http::Error>,
    client: &hyper::Client<HttpsConnector<HttpConnector>>,
    return_logic: fn(bytes: Bytes) -> Result<Return, Error>,
) -> Result<Return, Error> {
    let Ok(req) = request else {
        return Err(Error::InvalidRequest);
    };

    match client.request(req).await {
        Ok(mut res) => {
            let body = res.body_mut();
            let bytes = hyper::body::to_bytes(body).await;
            match bytes {
                Ok(bytes) => return_logic(bytes),
                Err(err) => panic!("{}", err),
            }
        }
        Err(err) => {
            if err.is_connect() {
                Err(running_error)
            } else {
                panic!("{}", err)
            }
        }
    }
}

#[cfg(any(feature = "in_game", feature = "rest"))]
pub(crate) fn uri_builder(url: &str, endpoint: &str) -> Result<Uri, Error> {
    uri::Builder::new()
        .scheme("https")
        .authority(url.as_bytes())
        .path_and_query(endpoint)
        .build()
        .map_or(Err(Error::InvalidRequest), Ok)
}
