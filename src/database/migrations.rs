use rusqlite::{Result};
use crate::database::connection::DB_CONNECTION;
use crate::files::extensions_file::get_extensions;
use crate::models::Extension;

pub fn run_migrations() -> Result<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    conn.execute("
        CREATE TABLE IF NOT EXISTS extension (
            id INTEGER PRIMARY KEY,
            tld VARCHAR(20) NOT NULL,
            name VARCHAR(70) NOT NULL,
            selected TINYINT(1) NOT NULL
        )",
        [],
    )?;

    let extensions = get_extensions();

    for extension in extensions.iter() {
        conn.execute(
            "INSERT INTO extension (tld, name, selected) VALUES (?1, ?2, ?3)",
            (&extension.tld, &extension.name, &extension.selected),
        )?;
    }

    let mut stmt = conn.prepare("SELECT id, tld, name, selected FROM extension")?;
    let extension_iter = stmt.query_map([], |row| {
        Ok(Extension {
            tld: row.get(1)?,
            name: row.get(2)?,
            selected: row.get(3)?,
        })
    })?;

    for extension in extension_iter {
        println!("Found extension {:?}", extension?);
    }

    Ok(())
}
