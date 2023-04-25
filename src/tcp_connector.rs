use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::string::FromUtf8Error;

pub fn list_for_clients(ip_address: &str, on_port: &str) -> TcpListener {
    let listener = TcpListener::bind(format!("{}:{}", ip_address, on_port)).unwrap();
    println!("Server listening port {}", &on_port);
    listener
}

pub fn send_command_to_client(mut stream: &TcpStream, command: &String) -> Result<String, FromUtf8Error> {
    stream.write(&command.as_bytes()).unwrap();

    let mut data = [0; 1024];
    stream.read(&mut data).unwrap();
    String::from_utf8(data.to_vec())
}