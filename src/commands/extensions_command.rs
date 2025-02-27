use crate::database::extensions_api::find_all_extensions;
use crate::tables::extensions_table::display_extensions;
use crate::models::Extension;

pub fn handle_extensions() {
  let extensions: Vec<Extension> = find_all_extensions();
  display_extensions(extensions).expect("An error occurred while displaying extensions");
}
