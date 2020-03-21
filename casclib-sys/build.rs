extern crate cmake;

use std::env;

fn main() {
    // Gets CacsLib source path from env CASCLIB_DIR
    let casclib_path = env::var("CASCLIB_DIR").unwrap_or("CascLib".to_string());

    println!("cargo:rerun-if-changed={}", casclib_path);

    // Builds CascLib using cmake
    let dst = cmake::Config::new(&casclib_path)
        .always_configure(true)
        .build();

    let lib = dst.join("lib");

    println!("cargo:rustc-link-search=native={}", lib.display());
    println!("cargo:rustc-link-lib=casc");
}
