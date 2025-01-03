use crate::extensions_ui::{display_extensions, Extension};
use serde_json;
use std::fs::File;
use std::io::Read;

pub fn handle_extensions() {
  println!("Handle domain extensions");
  let extensions: Vec<Extension> = get_extensions();
  display_extensions(extensions).expect("An error occurred while displaying extensions");
}

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
