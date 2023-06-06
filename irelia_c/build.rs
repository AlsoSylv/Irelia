#![feature(string_remove_matches)]

use std::{env, path::PathBuf};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_no_includes()
        .with_language(cbindgen::Language::C)
        .rename_item("CRT", "RT")
        .rename_item("CRequestClient", "RequestClient")
        .rename_item("CLCUClient", "LCUClient")
        .rename_item("CLCUResponse", "LCUResponse")
        .rename_item("CFuture", "Future")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");

    c_flags()
}

fn c_flags() {
    let include_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut lib_dir = include_dir.clone();
    lib_dir.remove_matches("irelia_c");

    let mut shared_object_dir = PathBuf::from(&lib_dir);

    shared_object_dir.push("target");
    shared_object_dir.push(env::var("PROFILE").unwrap());

    let shared_object_dir = shared_object_dir.as_path().to_string_lossy();

    println!(
        "cargo:rustc-env=INLINE_C_RS_CFLAGS=-I{I} -L{L} -D_DEBUG -D_CRT_SECURE_NO_WARNINGS",
        I = include_dir,
        L = shared_object_dir,
    );

    #[cfg(target_os = "linux")]
    println!(
        "cargo:rustc-env=INLINE_C_RS_LDFLAGS={shared_object_dir}/{lib}",
        shared_object_dir = shared_object_dir,
        lib = "libirelia_c.so",
    );

    #[cfg(target_os = "windows")]
    println!(
        "cargo:rustc-env=INLINE_C_RS_LDFLAGS={shared_object_dir}/{lib}",
        shared_object_dir = shared_object_dir,
        lib = "libirelia_c.dylib",
    );
}
