use std::fmt::Error;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io;
use std::io::stdin;
use std::num::ParseIntError;
use std::sync::{Arc, Mutex, MutexGuard};

mod tcp_connector;
mod tcp_streams_worker;

fn main() {
    let port: String = String::from("8888");
    let ip_address: String = String::from("0.0.0.0");
    let listener = tcp_connector::list_for_clients(&ip_address, &port);

    let streams: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    let streams_clone = Arc::clone(&streams);
    thread::spawn(move || listen_for_connections_and_push_to_vector(&listener, &streams_clone));

    open_main_menu(&streams);
}

fn open_main_menu(streams: &Arc<Mutex<Vec<TcpStream>>>) {
    while true {
        println!("\n\
---------------------\t
1: Check Connections\n
2: Connect To\t
---------------------");
        let mut menu_option = String::new();
        stdin().read_line(&mut menu_option).unwrap();
        match menu_option.trim() {
            "1" => {
                tcp_streams_worker::print_connections(&streams);
            }
            "2" => {
                // if !tcp_streams_worker::are_there_streams(&streams) {
                //     println!("There is no available connections");
                //     continue;
                // }

                println!("Enter order number to use connection");
                tcp_streams_worker::print_connections(&streams);
                let mut number: String = String::new();
                stdin().read_line(&mut number).unwrap();

                let mut index: usize = 0;
                let index_result: Result<usize, ParseIntError> = number.trim().parse();
                match index_result {
                    Ok(num) => {
                        index = num - 1;
                    }
                    Err(_) => {
                        println!("Parse error");
                        continue;
                    }
                }

                let stream: TcpStream = tcp_streams_worker::get_stream_by_index(
                    &streams,
                    index,
                );

                println!("Using the stream {}", stream.peer_addr().unwrap());
                println!("Enter the command (write 'break' to exit from connection)");

                while true {
                    let mut command: String = String::new();
                    stdin().read_line(&mut command).unwrap();

                    if command.trim() == "break" {
                        break;
                    }

                    match tcp_connector::send_command_to_client(&stream, &command) {
                        Ok(result) => {
                            println!("{}", result);
                        }
                        Err(e) => {
                            println!("Error - {}", e.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn listen_for_connections_and_push_to_vector(listener: &TcpListener, streams: &Arc<Mutex<Vec<TcpStream>>>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                let mut streams = streams.lock().unwrap();
                streams.push(stream);
            }
            Err(e) => {
                println!("Error {}", e)
            }
        }
    }
}
