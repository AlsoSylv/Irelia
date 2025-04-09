const RIOT_PEM: &[u8] = include_bytes!("src/riotgames.pem");

#[cfg(any(
    feature = "__hyper_rustls",
    feature = "__reqwest_rustls",
    feature = "ws_rustls"
))]
fn main() {
    use quote::quote;
    use std::env;
    use std::ops::Deref;
    use std::path::Path;

    let Ok(Some((pem, _))) = rustls_pemfile::read_one_from_slice(RIOT_PEM) else {
        panic!("Failed to decode pem file, this should never happen");
    };

    // Get it in the proper format
    let rustls_pemfile::Item::X509Certificate(pem) = pem else {
        unreachable!("This is an X509 Cert from riot, this should never be in any other format")
    };

    let Ok(anchor) = webpki::anchor_from_trusted_cert(&pem) else {
        panic!("The cert was from Rustls, so it should be valid")
    };

    let subject = anchor.subject.deref();
    let public_info = anchor.subject_public_key_info.deref();
    let name = anchor.name_constraints.as_deref();

    let name = name.map_or_else(
        || quote! { None },
        |slice| quote! { Some(Der::from_slice([#(#slice),*])) },
    );

    let tokens = quote! {
        pub(super) mod cert {
            use rustls::pki_types::{Der, TrustAnchor};
            const SUBJECT: Der = Der::from_slice(&[#(#subject),*]);
            const SUBJECT_PUBLIC_KEY_INFO: Der = Der::from_slice(&[#(#public_info),*]);
            const NAME_CONSTRAINTS: Option<Der> = #name;

            pub const DECODED_CERT: TrustAnchor = TrustAnchor {
                subject: SUBJECT,
                subject_public_key_info: SUBJECT_PUBLIC_KEY_INFO,
                name_constraints: NAME_CONSTRAINTS,
            };
        }
    };

    let tokens = if cfg!(debug_assertions) {
        prettyplease::unparse(&syn::parse2(tokens).unwrap())
    } else {
        tokens.to_string()
    };

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let path = Path::new(&out_dir).join("riot_games_const_pem.rs");

    std::fs::write(&path, tokens).unwrap();
}

#[cfg(any(
    feature = "__hyper_nativetls",
    feature = "__reqwest_nativetls",
    feature = "ws_nativetls"
))]
fn main() {
    use std::env;
    use std::path::Path;

    use quote::quote;

    let Ok(decoded) = native_tls::Certificate::from_pem(RIOT_PEM) else {
        panic!("Failed to decode pem file, this should never happen");
    };

    let Ok(der) = decoded.to_der() else {
        panic!("Failed to convert to der")
    };

    let tokens = quote! {
        pub(super) mod cert {
            pub const PEM_FILE: &[u8] = &[#(#der),*];
        }
    };

    let tokens = if cfg!(debug_assertions) {
        prettyplease::unparse(&syn::parse2(tokens).unwrap())
    } else {
        tokens.to_string()
    };

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let path = Path::new(&out_dir).join("riot_games_const_pem.rs");

    std::fs::write(&path, tokens).unwrap();
}

#[cfg(not(any(
    feature = "__hyper_rustls",
    feature = "__reqwest_rustls",
    feature = "__hyper_nativetls",
    feature = "__reqwest_nativetls",
    feature = "ws_rustls",
    feature = "ws_nativetls",
)))]
fn main() {}
