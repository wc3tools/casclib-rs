extern crate casclib;

const SC2_PATH: &'static str = r#"C:\Program Files (x86)\StarCraft II\SC2Data"#;
const TEST_FILE: &'static str = r#"campaigns\liberty.sc2campaign\base.sc2maps\maps\challenges\advancedcommand.sc2map\base.sc2data\GameData\UnitData.xml"#;

#[test]
fn test_all() {
  use std::io::Cursor;

  let storage = casclib::open(SC2_PATH).unwrap();
  let count = storage.get_file_count();
  assert!(count > 0);

  let mut walked = 0;
  let mut galaxy_file_count = 0;
  for r in storage.files() {
    walked = walked + 1;
    let entry = r.expect("file entry");
    let name = entry.get_name();
    if name.ends_with(".galaxy") {
      galaxy_file_count = galaxy_file_count + 1;
    }
  }

  assert!(galaxy_file_count > 0);
  
  let mut buffer = Cursor::<Vec<u8>>::new(vec![0; 0x1000]);
  let file = storage.entry(TEST_FILE).open().unwrap();
  let expected_content = include_bytes!("UnitData.xml");
  assert_eq!(file.get_size() as usize, expected_content.len());
  file.extract(&mut buffer).unwrap();
  let pos = buffer.position() as usize;
  assert_eq!(&buffer.get_ref()[0..pos], &expected_content[..]);
}