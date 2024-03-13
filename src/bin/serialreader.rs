use std::env;
use idtap::lookup_name_by_serial;
use rusqlite::{Connection, Result};
use std::io::{self, BufRead};
use std::time::Duration;
use std::fmt::Error;


fn main() -> Result<()> {
	// Read the serial port name from the command line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <serial_port>", args[0]);
        return Ok(());
    }

    let port_name = &args[1];
    println!("Serial port is {}", port_name);
	// let port_name = "/dev/pts/25";

    let conn = Connection::open("your_database.sqlite")?;
    let port = serialport::new(port_name, 9600)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(5000));

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
