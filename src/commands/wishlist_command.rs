use crate::database::wishlist_api::find_wishlist;
use crate::tables::domains_table::display_domains;
use crate::models::Domain;

pub fn handle_wishlist() {
  let wishlist: Vec<Domain> = find_wishlist();
  display_domains(wishlist).expect("An error occurred while displaying wishlisted domains");
}
