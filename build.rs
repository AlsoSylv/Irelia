#[cfg(feature = "c")]
fn main() {
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}

#[cfg(not(feature = "c"))]
fn main() {}
