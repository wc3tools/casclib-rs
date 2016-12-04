# casclib-rs
Safe [CascLib](https://github.com/ladislav-zezula/CascLib) bindings for Rust

```rust
let storage = open(r#"C:\Program Files (x86)\StarCraft II\SC2Data"#).unwrap();

// walk all files for r in storage.files() { let entry = r.expect("file entry"); let name = entry.get_name(); if name.ends_with(".galaxy") { // found a galaxy file!

use casclib::{open};
  }
}

// extract a file by path
let file = storage.entry(TEST_FILE).open().unwrap();
file.extract(writer).unwrap();
```
