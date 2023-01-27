use tokio_native_tls::native_tls::{Certificate, TlsConnector};

/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) fn setup_tls_connector() -> TlsConnector {
    let cert = Certificate::from_pem(include_bytes!("../riotgames.pem")).unwrap();
    TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap()
}
