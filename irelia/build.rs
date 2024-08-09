use quote::quote;
use std::env;
use std::ops::Deref;
use std::path::Path;

const RIOT_PEM: &[u8] = include_bytes!("src/riotgames.pem");

fn main() {
    // Make it rustls compatible
    let (pem, _) = rustls_pemfile::read_one_from_slice(RIOT_PEM)
        .unwrap()
        .unwrap();
    // Get it in the proper format
    let rustls_pemfile::Item::X509Certificate(pem) = pem else {
        unreachable!()
    };

    let anchor = webpki::anchor_from_trusted_cert(&pem).unwrap();

    let subject = anchor.subject.deref();
    let public_info = anchor.subject_public_key_info.deref();
    let name = anchor.name_constraints.as_deref();

    let name = if let Some(name) = name {
        quote! { Some(Der::from_slice([#(#name),*])) }
    } else {
        quote! { None }
    };

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
