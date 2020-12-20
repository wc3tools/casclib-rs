const STROAGE_PATH: &'static str = r#"C:\Program Files (x86)\Warcraft III\Data"#;
const TEST_FILE: &'static str = r#"war3.w3mod:scripts\blizzard.j"#;

#[test]
fn test_all() {
    let storage = casclib::open(STROAGE_PATH).unwrap();
    let count = storage.file_count();
    assert!(count > 0);

    let mut walked = 0;
    let mut map_file_count = 0;
    for r in storage.files_with_mask("war3.w3mod:maps") {
        walked = walked + 1;
        let entry = r.expect("file entry");
        let _name = entry.get_name();
        map_file_count = map_file_count + 1;
    }

    assert!(map_file_count > 0);

    let _bj = storage.entry(TEST_FILE).open().unwrap();
}

#[cfg(target_os = "windows")]
#[test]
fn test_read_unicode() {
    use widestring::U16CString;
    use std::os::windows::ffi::OsStringExt;
    use std::ffi::OsString;

    let storage = casclib::open(OsString::from_wide(&U16CString::from_str(r#"C:\Program Files (x86)\Warcraft III中文\Data"#).unwrap().into_vec())).unwrap();
    let count = storage.file_count();

    assert!(count > 0);

    let mut walked = 0;
    let mut map_file_count = 0;
    for r in storage.files_with_mask("war3.w3mod:maps") {
        walked = walked + 1;
        let entry = r.expect("file entry");
        let _name = entry.get_name();
        map_file_count = map_file_count + 1;
    }

    assert!(map_file_count > 0);

    let _bj = storage.entry(TEST_FILE).open().unwrap();
}
