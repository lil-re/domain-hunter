use serde_json;
use std::fs::File;
use std::io::Read;
use crate::domains_table::display_domains;
use crate::models::Domain;

pub fn handle_wishlist() {
  println!("Handle domains wishlist");
  let mut wishlist: Vec<Domain> = get_wishlist();
  format_wishlisted_domain(&mut wishlist);
  display_domains(wishlist).expect("An error occurred while displaying wishlisted domains");
}

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

pub fn format_wishlisted_domain(wishlist: &mut Vec<Domain>) {
  for domain in wishlist {
   domain.selected = true;
  }
}
