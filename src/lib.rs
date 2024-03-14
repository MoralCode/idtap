use rusqlite::{Connection, Result};

struct Person {
    name: String,
    serial_number: i32,
}

fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
                serial_number INTEGER PRIMARY KEY
                name TEXT NOT NULL,
            )",
        [],
    )?;

    Ok(())
}

fn check_database_initialized(conn: &Connection) -> Result<bool> {
    return table_exists(conn, "people");
}

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
        table_name
    );
    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |_| Ok(()))?;

    let exists = rows.count()? > 0;
    Ok(exists)
}


pub fn lookup_name_by_serial(serial: i32, conn: &Connection) -> Result<Option<String>> {
    match check_database_initialized(conn) {
        Ok(result) => {
            if (!result) {
                initialize_database(conn);
            }
        }
        Err(e) => eprintln!("Error checking DB initialization: {}", e),
    }

    let mut stmt = conn.prepare("SELECT name FROM people WHERE serial_number = ?")?;
    let mut rows = stmt.query([serial])?;

    if let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        Ok(Some(name))
    } else {
        Ok(None)
    }
}

fn main() -> Result<()> {
    // Replace "your_database.sqlite" with the actual path to your SQLite database file.
    let conn = Connection::open("your_database.sqlite")?;

    // Initialize the database if it doesn't exist
    initialize_database(&conn)?;

    // Replace "123456" with the actual serial number you want to look up.
    let serial_to_lookup = 123456;

    match lookup_name_by_serial(serial_to_lookup, &conn)? {
        Some(name) => println!("Name for serial number {}: {}", serial_to_lookup, name),
        None => println!("No name found for serial number: {}", serial_to_lookup),
    }

    Ok(())
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
