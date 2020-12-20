#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[cfg(target_os = "windows")]
include!("./bindings_windows.rs");
#[cfg(target_os = "linux")]
include!("./bindings_linux.rs");
#[cfg(target_os = "macos")]
include!("./bindings_macos.rs");

pub type ErrorCode = DWORD;