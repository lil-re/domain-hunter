use crate::database::connection::DB_CONNECTION;
use crate::models::Domain;

pub fn add_to_wishlist(domain: &Domain) -> Option<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let response = conn.execute(
        "INSERT INTO wishlist (tld, domain, status, selected) VALUES (?1, ?2, ?3, ?4)",
        (&domain.tld, &domain.domain, &domain.status, &domain.selected),
    )
        .map_err(|e| format!("Failed to add domain to wishlist: {}", e));

    match response {
        Ok(_) => Some(()),
        Err(error) => {
            println!("WISHLIST API => {}", error);
            None
        }
    }
}

pub fn remove_from_wishlist(domain: &Domain) -> Option<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let response = conn.execute(
        "DELETE FROM wishlist WHERE tld = ?1 AND domain = ?2",
        (&domain.tld, &domain.domain),
    )
        .map_err(|e| format!("Failed to update domain: {}", e));

    match response {
        Ok(_) => Some(()),
        Err(error) => {
            println!("WISHLIST API => {}", error);
            None
        }
    }
}

pub fn find_wishlist() -> Vec<Domain> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("SELECT * FROM wishlist") {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

    let domains_iter = stmt.query_map([], |row| {
        Ok(Domain {
            tld: row.get(1)?,
            domain: row.get(2)?,
            status: row.get(3)?,
            selected: row.get(4)?,
        })
    });

    let domains_result = match domains_iter {
        Ok(result) => result.collect::<Result<Vec<Domain>, rusqlite::Error>>(),
        Err(_) => Ok(vec![])
    };

    match domains_result {
        Ok(result) => result,
        Err(_) => vec![]
    }
}
