use rustls::pki_types::CertificateDer;

//noinspection SpellCheckingInspection
/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) fn setup_tls_connector() -> rustls::ClientConfig {
    const CERT: &[u8] = include_bytes!("../riotgames.pem");

    let cert = CertificateDer::from(CERT);
    let mut roots = rustls::RootCertStore::empty();
    roots.add(cert).unwrap();
    rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth()
}
