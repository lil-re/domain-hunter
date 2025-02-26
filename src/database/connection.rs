use rusqlite::{Connection, Result};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(
        match Connection::open_in_memory() {
            Ok(connection) => connection,
            Err(_) => panic!("Failed to establish connection with database")
        }
    );
}

pub fn establish_connection() -> Result<()> {
    let _conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");
    // Additional initialization code for the connection can go here if needed.
    Ok(())
}
