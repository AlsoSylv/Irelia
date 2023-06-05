use rustls::{ClientConfig, RootCertStore};
use std::sync::LazyLock;

/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) static TLS_CONFIG: LazyLock<rustls::ClientConfig> = LazyLock::new(|| {
    let mut cert: &[u8] = include_bytes!("../riotgames.pem");
    // let cert = Certificate(cert.to_vec());
    let cert = rustls_pemfile::certs(&mut cert).unwrap();
    let mut roots = RootCertStore::empty();
    let _ = roots.add_parsable_certificates(&cert);
    ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth()
});
