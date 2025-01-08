use crate::extensions_file::get_extensions;
use crate::extensions_table::display_extensions;
use crate::models::Extension;

pub fn handle_extensions() {
  let extensions: Vec<Extension> = get_extensions();
  display_extensions(extensions).expect("An error occurred while displaying extensions");
}
