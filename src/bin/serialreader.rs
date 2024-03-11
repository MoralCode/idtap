use idtap::lookup_name_by_serial;
use rusqlite::{Connection, Result};
use std::io::{self, BufRead};
use std::time::Duration;

fn main() -> Result<()> {
    // Replace "COM1" with the actual serial port name on your system.
    let port_name = "COM1";

    let conn = Connection::open("your_database.sqlite")?;
    let port = serialport::new(port_name, 9600)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(100));

    match port.open() {
        Ok(mut port) => {
            println!("Serial port opened successfully. Reading serial numbers...");

            // Replace this loop with your actual serial number reading logic.
            let reader = io::BufReader::new(&mut port);
            for line in reader.lines() {
                if let Ok(serial_number) = line.unwrap().parse::<i32>() {
                    // Assume you have already created a database connection (`conn`).
                    match lookup_name_by_serial(serial_number, &conn)? {
                        Some(name) => {
                            println!("Name for serial number {}: {}", serial_number, name)
                        }
                        None => println!("No name found for serial number: {}", serial_number),
                    }
                }
            }

            println!("Serial numbers read successfully.");
        }
        Err(e) => eprintln!("Error opening serial port: {}", e),
    }

    Ok(())
}
