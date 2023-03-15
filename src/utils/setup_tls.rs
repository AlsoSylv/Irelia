use once_cell::sync::Lazy;
use tokio_native_tls::native_tls::{Certificate, TlsConnector};

const CERT: &[u8] = include_bytes!("../riotgames.pem");

/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) static TLS_CONNECTOR: Lazy<TlsConnector> = Lazy::new(|| {
    let cert = Certificate::from_pem(CERT).unwrap();
    TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap()
});
