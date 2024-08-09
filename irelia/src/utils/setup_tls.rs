use std::sync::LazyLock;

static RUSTLS_CLIENT_CONFIG: LazyLock<rustls::ClientConfig> = LazyLock::new(connector_internal);

include!(concat!(env!("OUT_DIR"), "/riot_games_const_pem.rs"));

//noinspection SpellCheckingInspection
/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
fn connector_internal() -> rustls::ClientConfig {
    // Create a new empty cert store
    let roots = rustls::RootCertStore {
        roots: vec![cert::DECODED_CERT],
    };
    // Return the new client config with just the riot cert
    rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth()
}

pub fn connector() -> &'static rustls::ClientConfig {
    &RUSTLS_CLIENT_CONFIG
}

#[test]
fn test_connector() {
    connector();
}
