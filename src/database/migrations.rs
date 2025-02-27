use rusqlite::{Result};
use crate::database::extensions_api::create_extension;
use crate::database::connection::DB_CONNECTION;
use crate::models::Extension;
use crate::default_extensions::DEFAULT_EXTENSIONS;

pub fn run_migrations() -> Result<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    conn.execute("
        CREATE TABLE IF NOT EXISTS domain (
            id INTEGER PRIMARY KEY,
            tld VARCHAR(20) NOT NULL,
            domain VARCHAR(70) NOT NULL,
            status VARCHAR(70) NOT NULL,
            selected TINYINT(1) NOT NULL
        )",
        [],
    )?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS extension (
            id INTEGER PRIMARY KEY,
            tld VARCHAR(20) NOT NULL,
            name VARCHAR(70) NOT NULL,
            selected TINYINT(1) NOT NULL
        )",
        [],
    )?;

    let extensions = get_default_extensions();

    for extension in extensions.iter() {
        create_extension(&conn, &extension);
        // conn.execute(
        //     "INSERT INTO extension (tld, name, selected) VALUES (?1, ?2, ?3)",
        //     (&extension.tld, &extension.name, &extension.selected),
        // )?;
    }

    Ok(())
}

pub fn get_default_extensions() -> Vec<Extension> {
    // Transform content into a vector of Extension
    match serde_json::from_str(DEFAULT_EXTENSIONS) {
        Ok(result) => result,
        Err(error) => { panic!("{}", error) }
    }
}
