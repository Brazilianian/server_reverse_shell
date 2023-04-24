use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn try_to_ip_config(mut stream: TcpStream) {
    let command: String = String::from("/ipconfig");
    stream.write(&command.as_bytes()).unwrap();
    print!("Sent");

    let mut data: Vec<u8> = vec![];
    stream.read(&mut data).unwrap();
    print!("{}", String::from_utf8(data).unwrap());
}

fn main() {
    let port: String = String::from("8888");
    let ip_address: String = String::from("0.0.0.0:") + &port;
    let listener = TcpListener::bind(ip_address).unwrap();
    println!("Server listening port {}", &port);

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                try_to_ip_config(stream)
            }
            Err(e) => {
                println!("Error {}", e)
            }
        }
    }
}
