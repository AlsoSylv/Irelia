/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) fn setup_tls_connector() -> rustls::ClientConfig {
    let mut cert: &[u8] = include_bytes!("../riotgames.pem");
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_pemfile::certs(&mut cert) {
        let cert = cert.unwrap();
        roots.add(cert).unwrap();
    }
    rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth()
}
