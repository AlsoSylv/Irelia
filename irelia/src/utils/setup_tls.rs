//noinspection SpellCheckingInspection
/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) fn setup_tls_connector() -> rustls::ClientConfig {
    let mut cert: &[u8] = include_bytes!("../riotgames.pem");
    let pem = rustls_pemfile::read_one(&mut cert).unwrap().unwrap();
    let rustls_pemfile::Item::X509Certificate(pem) = pem else {
        unreachable!()
    };
    let mut roots = rustls::RootCertStore::empty();
    roots.add(pem).unwrap();
    rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth()
}
