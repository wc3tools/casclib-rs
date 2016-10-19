extern crate cmake;

use std::env;

fn main() {
    // Gets CacsLib source path from env CASCLIB_DIR
    let casclib_path = env::var("CASCLIB_DIR").unwrap_or("CascLib".to_string());

    println!("cargo:rerun-if-changed={}", casclib_path);

    // Builds CascLib using cmake
    let mut dst = cmake::Config::new(&casclib_path)
        .define("WITH_STATIC", "ON")
        .build();
    dst.push("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=casc");
}
