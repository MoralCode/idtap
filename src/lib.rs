use rusqlite::{Connection, Result};

struct Person {
    name: String,
    serial_number: i32,
}

pub fn lookup_name_by_serial(serial: i32, conn: &Connection) -> Result<Option<String>> {
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
