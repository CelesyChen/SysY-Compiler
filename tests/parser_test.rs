use sysy::parser::parse_file;
use std::path::{Path};
use std::fs::{read_dir};


#[test]
fn test_parser() {
  let dir = read_dir(Path::new("./testsets")).unwrap();
  for entry in dir {
    let entry = entry.unwrap();
    let path = entry.path();
    if !path.ends_with(".sy") {
      continue;
    }

    assert_eq!(parse_file(&path).unwrap(), "Ok".to_string()) ;
  }
}