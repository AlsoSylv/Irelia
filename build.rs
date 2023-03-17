#[cfg(any(feature = "c_rest", feature = "c_in_game", feature = "c_ws"))]
fn main() {
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::C)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}

#[cfg(not(any(feature = "c_rest", feature = "c_in_game", feature = "c_ws")))]
fn main() {}
