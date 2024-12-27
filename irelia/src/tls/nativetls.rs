use std::sync::LazyLock;

include!(concat!(env!("OUT_DIR"), "/riot_games_const_pem.rs"));

static NATIVE_TLS_CERTIFICATE: LazyLock<native_tls::TlsConnector> =
    LazyLock::new(connector_internal);

fn connector_internal() -> native_tls::TlsConnector {
    // This was turned into a der in the build script, so if this fails something is horribly wrong
    let cert = native_tls::Certificate::from_der(cert::PEM_FILE).unwrap();

    let connector = native_tls::TlsConnector::builder()
        .add_root_certificate(cert)
        .disable_built_in_roots(true)
        .build()
        .unwrap();

    connector
}

#[cfg(feature = "ws")]
pub use ws::*;

#[cfg(feature = "ws")]
mod ws {
    use super::NATIVE_TLS_CERTIFICATE;

    pub type TlsType = native_tls::TlsConnector;

    pub fn wrap_connector(tls: &TlsType) -> tungstenite::Connector {
        tungstenite::Connector::NativeTls(tls.clone())
    }

    pub fn connector() -> TlsType {
        NATIVE_TLS_CERTIFICATE.clone()
    }
}

#[cfg(any(feature = "rest", feature = "in_game", feature = "replay"))]
pub use http::*;

#[cfg(any(feature = "rest", feature = "in_game", feature = "replay"))]
mod http {
    use hyper_util::client::legacy::connect;

    use super::NATIVE_TLS_CERTIFICATE;

    pub type Connector = hyper_tls::HttpsConnector<connect::HttpConnector>;

    pub fn https_connector() -> Connector {
        let connector = NATIVE_TLS_CERTIFICATE.clone();
        let mut http = connect::HttpConnector::new();
        http.enforce_http(false);
        let mut https = hyper_tls::HttpsConnector::from((http, connector.clone().into()));
        https.https_only(true);
        https
    }
}
