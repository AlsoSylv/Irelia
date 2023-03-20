#[cfg(feature = "C")]
fn main() {
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::C)
        .with_crate(&crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("irelia.h");

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::Cxx)
        .with_crate(&crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("irelia.hpp");
}

#[cfg(not(feature = "C"))]
fn main() {}
