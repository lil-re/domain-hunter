use crate::domains_table::display_domains;
use crate::models::Domain;
use crate::wishlist_file::get_wishlist;

pub fn handle_wishlist() {
  println!("Handle domains wishlist");
  let wishlist: Vec<Domain> = get_wishlist();
  display_domains(wishlist).expect("An error occurred while displaying wishlisted domains");
}
