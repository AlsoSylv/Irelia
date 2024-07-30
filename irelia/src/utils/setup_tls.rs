static RUSTLS_CLIENT_CONFIG: LazyLock<rustls::ClientConfig> = LazyLock::new(connector_internal);

const CERT: &[u8] = include_bytes!("../riotgames.pem");

//noinspection SpellCheckingInspection
/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
fn connector_internal() -> rustls::ClientConfig {
    // Make it rustls compatible
    let (pem, _) = rustls_pemfile::read_one_from_slice(CERT).unwrap().unwrap();
    // Get it in the proper format
    let rustls_pemfile::Item::X509Certificate(pem) = pem else {
        unreachable!()
    };

    let anchor = webpki::anchor_from_trusted_cert(&pem).unwrap();

    // Create a new empty cert store
    let roots = rustls::RootCertStore {
        roots: vec![anchor.to_owned()],
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

struct LazyLock<T, F = fn() -> T> {
    data: std::sync::OnceLock<T>,
    f: F,
}

impl<T, F> LazyLock<T, F> {
    const fn new(f: F) -> Self {
        Self {
            data: std::sync::OnceLock::new(),
            f,
        }
    }
}

impl<T> std::ops::Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.get_or_init(self.f)
    }
}
