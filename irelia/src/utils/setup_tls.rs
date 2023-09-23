/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) fn setup_tls_connector() -> rustls::ClientConfig {
    let mut cert: &[u8] = include_bytes!("../riotgames.pem");
    // let cert = Certificate(cert.to_vec());
    let cert = rustls_pemfile::certs(&mut cert).unwrap();
    let mut roots = rustls::RootCertStore::empty();
    let _ = roots.add_parsable_certificates(&cert);
    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth()
}
