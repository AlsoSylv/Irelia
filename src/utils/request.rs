use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_native_tls::native_tls::{Certificate, TlsConnector};

pub(crate) fn setup_hyper_client() -> Result<Client<HttpsConnector<HttpConnector>>, ()> {
    let tls = setup_tls_connector();
    let tokio_tls = tokio_native_tls::TlsConnector::from(tls);
    let mut http = HttpConnector::new();
    http.enforce_http(false);
    let https = HttpsConnector::from((http, tokio_tls));
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    Ok(client)
}

pub(crate) fn setup_tls_connector() -> TlsConnector {
    let cert = Certificate::from_pem(include_bytes!("../riotgames.pem")).unwrap();
    TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap()
}
