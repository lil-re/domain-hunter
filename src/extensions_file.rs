use std::fs::File;
use std::io::Read;
use crate::models::Extension;

pub fn get_extensions() -> Vec<Extension> {
  let mut contents = String::new();
  let mut file = match File::open("extensions.json") {
    Ok(result) => result,
    Err(error) => { panic!("{}", error) }
  };

  match file.read_to_string(&mut contents) {
    Ok(result) => result,
    Err(error) => { panic!("{}", error) }
  };

  match serde_json::from_str(&contents) {
    Ok(result) => result,
    Err(error) => { panic!("{}", error) }
  }
}