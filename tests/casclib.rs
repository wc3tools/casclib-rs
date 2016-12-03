extern crate casclib;

const SC2_PATH: &'static str = r#"C:\Program Files (x86)\StarCraft II\SC2Data"#;

#[test]
fn test_all() {
    let storage = casclib::open(SC2_PATH).unwrap();
    let count = storage.get_file_count();
    assert!(count > 0);

    let mut walked = 0;
    let mut galaxy_file_count = 0;
    for r in storage.files() {
      walked = walked + 1;
      let entry = r.expect("file entry");
      if entry.get_name().ends_with(".galaxy") {
        galaxy_file_count = galaxy_file_count + 1;
      }
    }

    assert!(galaxy_file_count > 0);
}