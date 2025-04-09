use std::sync::LazyLock;

static RUSTLS_CLIENT_CONFIG: LazyLock<rustls::ClientConfig> = LazyLock::new(connector_internal);

include!(concat!(env!("OUT_DIR"), "/riot_games_const_pem.rs"));

//noinspection SpellCheckingInspection
/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
pub(crate) fn connector_internal() -> rustls::ClientConfig {
    // Create a new empty cert store
    let roots = rustls::RootCertStore {
        roots: vec![cert::DECODED_CERT],
    };
    // Return the new client config with just the riot cert
    rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth()
}

#[cfg(feature = "ws")]
pub use ws::*;

#[cfg(feature = "ws")]
mod ws {
    use std::sync::Arc;

    use super::RUSTLS_CLIENT_CONFIG;

    pub type TlsType = Arc<rustls::ClientConfig>;

    pub fn wrap_connector(tls: &Arc<rustls::ClientConfig>) -> tungstenite::Connector {
        tungstenite::Connector::Rustls(tls.clone())
    }

    pub fn connector() -> TlsType {
        Arc::new(RUSTLS_CLIENT_CONFIG.clone())
    }
}

#[cfg(feature = "__hyper_rustls")]
pub use http::*;

#[cfg(feature = "__hyper_rustls")]
mod http {
    use hyper_util::client::legacy::connect;

    use super::RUSTLS_CLIENT_CONFIG;

    pub type Connector = hyper_rustls::HttpsConnector<connect::HttpConnector>;

    pub fn https_connector() -> Connector {
        // Get a client config using the riotgames.pem file
        let tls = RUSTLS_CLIENT_CONFIG.clone();
        // Set up an HTTPS only client, with just the client config
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls.clone())
            .https_only()
            .enable_http1()
            .build()
    }
}
