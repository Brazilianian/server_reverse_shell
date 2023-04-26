use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub fn print_connections(streams: &Arc<Mutex<Vec<TcpStream>>>) {
    check_are_connected(streams);

    if !are_there_streams(streams) {
        println!("There is no available connections");
        return;
    }


    let streams = streams.lock().unwrap();
    for i in 0..streams.len() {
        let stream = streams.get(i).unwrap();
        println!("{}) - {}", i + 1, stream.peer_addr().unwrap());
    }
}

fn check_are_connected(streams: &Arc<Mutex<Vec<TcpStream>>>) {
    let mut streams = streams.lock().unwrap();

    for mut i in 0..streams.len() {
        let mut buf = "CHECK".as_bytes();
        match streams.get(i) {
            Some(mut stream) => {
                match stream.write(&mut buf) {
                    Ok(_) => {}
                    Err(_) => {
                        streams.remove(i);
                        i = i.wrapping_sub(1);
                    }
                }
            }
            None => {
                i = i.wrapping_sub(1);
                streams.remove(i);
            }
        }
    }
}

pub fn are_there_streams(streams: &Arc<Mutex<Vec<TcpStream>>>) -> bool {
    let mut streams = streams.lock().unwrap();
    (!streams.is_empty() && streams.len() != 0)
}

pub fn get_stream_by_index(streams: &Arc<Mutex<Vec<TcpStream>>>, index: usize) -> TcpStream {
    let mut streams = streams.lock().unwrap();
    let stream = streams[index].try_clone().unwrap();
    stream
}