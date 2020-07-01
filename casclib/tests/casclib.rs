const STROAGE_PATH: &'static str = r#"C:\Program Files (x86)\Warcraft III\Data"#;
const TEST_FILE: &'static str = r#"war3.w3mod:scripts\blizzard.j"#;

#[test]
fn test_all() {
    use std::io::Cursor;

    let storage = casclib::open(STROAGE_PATH).unwrap();
    let count = storage.get_file_count();
    assert!(count > 0);

    let mut walked = 0;
    let mut map_file_count = 0;
    for r in storage.files_with_mask("war3.w3mod:maps") {
        walked = walked + 1;
        let entry = r.expect("file entry");
        let name = entry.get_name();
        map_file_count = map_file_count + 1;
    }

    assert!(map_file_count > 0);

    let _bj = storage.entry(TEST_FILE).open().unwrap();
}
