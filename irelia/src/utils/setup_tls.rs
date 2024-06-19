static RUSTLS_CLIENT_CONFIG: LazyLock<rustls::ClientConfig> = LazyLock::new(connector_internal);

//noinspection SpellCheckingInspection
/// Setups up the TLS connector, this is outside the hyper client as
/// It is required inside the websocket implementation
fn connector_internal() -> rustls::ClientConfig {
    // Get a copy of the pem file
    let mut cert: &[u8] = include_bytes!("../riotgames.pem");
    // Make it rustls compatible
    let pem = rustls_pemfile::read_one(&mut cert).unwrap().unwrap();
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

pub(crate) fn connector() -> &'static rustls::ClientConfig {
    &RUSTLS_CLIENT_CONFIG
}

struct LazyLock<T, F = fn() -> T> {
    data: std::sync::OnceLock<T>,
    f: F,
}

impl<T, F> LazyLock<T, F> {
    const fn new(f: F) -> LazyLock<T, F> {
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
