//noinspection SpellCheckingInspection
/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) fn setup_tls_connector() -> rustls::ClientConfig {
    // Get a copy of the pem file
    let mut cert: &[u8] = include_bytes!("../riotgames.pem");
    // Make it rustls compatible
    let pem = rustls_pemfile::read_one(&mut cert).unwrap().unwrap();
    // Get it in the proper format
    let rustls_pemfile::Item::X509Certificate(pem) = pem else {
        unreachable!()
    };
    // Create a new empty cert store
    let mut roots = rustls::RootCertStore::empty();
    // Add the pem file
    roots.add(pem).unwrap();
    // Return the new client config with just the riot cert
    rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth()
}
