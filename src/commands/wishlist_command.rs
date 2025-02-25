use crate::wishlist_file::get_wishlist;
use crate::tables::domains_table::display_domains;
use crate::models::Domain;

pub fn handle_wishlist() {
  let wishlist: Vec<Domain> = get_wishlist();
  display_domains(wishlist).expect("An error occurred while displaying wishlisted domains");
}
