use serde_json;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use crate::models::Domain;

pub fn get_wishlist() -> Vec<Domain> {
  let mut contents = String::new();
  let mut file = match File::open("wishlist.json") {
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

pub fn save_wishlist(wishlist: Vec<Domain>) {
  let json: String = serde_json::to_string(&wishlist).unwrap();
  let file_path = "wishlist.json";
  let mut file: File = OpenOptions::new()
      .create(true)   // Creates the file if it doesn't exist
      .write(true)    // Allows writing to the file
      .truncate(true) // Ensures the file content is replaced
      .open(file_path)
      .unwrap_or_else(|error| { panic!("{}", error) });

  let _ = file.write(json.as_bytes());
}

pub fn add_to_wishlist(domain: Domain) {
  let mut wishlist = get_wishlist();
  wishlist.push(domain);
  save_wishlist(wishlist);
}

pub fn remove_from_wishlist(domain: Domain) {
  let wishlist = get_wishlist();
  let new_wishlist = wishlist
      .into_iter()
      .filter(|d| d.domain_name() != domain.domain_name())
      .collect();

  save_wishlist(new_wishlist);
}
