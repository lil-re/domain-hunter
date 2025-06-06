use std::sync::MutexGuard;
use rusqlite::{Connection, Statement};
use crate::database::connection::DB_CONNECTION;
use crate::models::Extension;

/// Create a domain name extension
pub fn create_extension(conn: &MutexGuard<Connection>, extension: &Extension) -> Option<()> {
    let response = conn.execute(
        "INSERT INTO extension (tld, name, selected) VALUES (?1, ?2, ?3)",
        (&extension.tld, &extension.name, &extension.selected),
    )
        .map_err(|e| format!("Failed to insert extension: {}", e));

    match response {
        Ok(_) => Some(()),
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

/// Update a domain name extension
pub fn update_extension(extension: &Extension) -> Option<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let response = conn.execute(
        "UPDATE extension SET selected = ?1 WHERE tld = ?2",
        (&extension.selected, &extension.tld),
    )
        .map_err(|e| format!("Failed to update extension: {}", e));

    match response {
        Ok(_) => Some(()),
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

/// Get all domain extensions
pub fn find_all_extensions() -> Vec<Extension> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("SELECT id, tld, name, selected FROM extension") {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

    handle_extensions_result(&mut stmt)
}

/// Get domain extensions selected by user
pub fn find_selected_extensions() -> Vec<Extension> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("SELECT id, tld, name, selected FROM extension WHERE selected = TRUE") {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

    handle_extensions_result(&mut stmt)
}

/// Trigger a SQL query to get extensions
fn handle_extensions_result(stmt: &mut Statement) -> Vec<Extension> {
    let extensions_iter = stmt.query_map([], |row| {
        Ok(Extension {
            tld: row.get(1)?,
            name: row.get(2)?,
            selected: row.get(3)?,
        })
    });

    let extensions_result = match extensions_iter {
        Ok(result) => result.collect::<Result<Vec<Extension>, rusqlite::Error>>(),
        Err(_) => Ok(vec![])
    };

    match extensions_result {
        Ok(result) => result,
        Err(_) => vec![]
    }
}
