use std::sync::MutexGuard;
use rusqlite::Connection;
use crate::database::connection::DB_CONNECTION;
use crate::models::Extension;

pub fn create_extension(conn: &MutexGuard<Connection>, extension: &Extension) -> Option<bool> {
    let response = conn.execute(
        "INSERT INTO extension (tld, name, selected) VALUES (?1, ?2, ?3)",
        (&extension.tld, &extension.name, &extension.selected),
    )
        .map_err(|e| format!("Failed to insert extension: {}", e));

    match response {
        Ok(_) => Some(true),
        Err(error) => {
            println!("{}", error);
            Some(false)
        }
    }
}

pub fn find_all_extensions() -> Vec<Extension> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("SELECT id, tld, name, selected FROM extension") {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

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

pub fn find_selected_extensions() -> Vec<Extension> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("SELECT id, tld, name, selected FROM extension WHERE selected = TRUE") {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

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

pub fn update_extension(extension: &Extension) -> Option<bool> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let response = conn.execute(
        "UPDATE extension SET selected = ?1 WHERE tld = ?2",
        (&extension.selected, &extension.tld),
    )
        .map_err(|e| format!("Failed to update extension: {}", e));

    match response {
        Ok(_) => Some(true),
        Err(error) => {
            println!("{}", error);
            Some(false)
        }
    }
}
