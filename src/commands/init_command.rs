use crate::database::migrations::run_migrations;

pub fn handle_init() {
  match run_migrations() {
    Ok(_) => println!("Ok"),
    Err(e) => println!("{}", e)
  }
}
