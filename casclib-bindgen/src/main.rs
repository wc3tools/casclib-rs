use std::path::PathBuf;

fn main() {
  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .clang_arg("-I./deps/CascLib/src")
    .header("./casclib-bindgen/src/wrapper.hpp")
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .whitelist_type("^CASC.+")
    .whitelist_function("^Casc.+")
    .whitelist_function("SetCascError")
    .whitelist_function("GetCascError")
    .whitelist_var("^CASC.+");

  let bindings = bindings
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from("./casclib-sys/src");
  #[cfg(target_os = "windows")]
  let file_name = "bindings_windows.rs";
  #[cfg(target_os = "linux")]
  let file_name = "bindings_linux.rs";
  #[cfg(target_os = "macos")]
  let file_name = "bindings_macos.rs";
  bindings
    .write_to_file(out_path.join(file_name))
    .expect("Couldn't write bindings!");
}