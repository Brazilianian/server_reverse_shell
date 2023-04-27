use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::string::FromUtf8Error;
use log::Level;
use crate::logger::write_log;

pub fn list_for_clients(ip_address: &str, on_port: &str) -> TcpListener {
    let listener = TcpListener::bind(format!("{}:{}", ip_address, on_port)).unwrap();
    println!("Server listening port {}", &on_port);
    write_log(String::from("Server stared"), Level::Info);

    listener
}

pub fn send_command_to_client(mut stream: &TcpStream, command: &String) -> Result<String, FromUtf8Error> {
    match stream.write(&command.as_bytes()) {
        Ok(_) => {
            write_log(format!("Sent {} to {}", command, stream.peer_addr().unwrap()), Level::Info);
        }
        Err(_) => {
            write_log(format!("Failed to send {} to {}", command, stream.peer_addr().unwrap()), Level::Error);
            println!("Error - Disconnected");
        }
    }

    let mut data = [0; 16384];
    match stream.read(&mut data) {
        Ok(_) => {
            write_log(format!("Got {} from {}", String::from_utf8(data.to_vec()).unwrap(), stream.peer_addr().unwrap()), Level::Info)
        }
        Err(e) => {
            write_log(format!("Failed to get data from {}", stream.peer_addr().unwrap()), Level::Error);
            print!("Error - {}", e.to_string());
        }
    }
    String::from_utf8(data.to_vec())
}