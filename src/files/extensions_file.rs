use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::default_extensions::DEFAULT_EXTENSIONS;
use crate::models::{Extension};

pub fn get_extensions() -> Vec<Extension> {
  let mut contents = String::new();

  // Try to open the extensions file
  match File::open("extensions.json") {
    Ok(mut file) => {
      // If file exists, retrieve its content
      match file.read_to_string(&mut contents) {
        Ok(result) => result,
        Err(error) =>  panic!("{}", error)
      };
    },
    Err(_error) => {
      // Otherwise, add a default extension list
      contents = DEFAULT_EXTENSIONS.parse().unwrap()
    }
  };

  // Transform content into a vector of Extension
  match serde_json::from_str(&contents) {
    Ok(result) => result,
    Err(error) => { panic!("{}", error) }
  }
}

pub fn save_extensions(extensions: &Vec<Extension>) {
  let json: String = serde_json::to_string(&extensions).unwrap();
  let file_path = "extensions.json";
  let mut file: File = OpenOptions::new()
      .create(true)   // Creates the file if it doesn't exist
      .write(true)    // Allows writing to the file
      .truncate(true) // Ensures the file content is replaced
      .open(file_path)
      .unwrap_or_else(|error| { panic!("{}", error) });

  let _ = file.write(json.as_bytes());
}
