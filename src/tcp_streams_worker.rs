use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub fn print_connections(streams: &Arc<Mutex<Vec<TcpStream>>>) {
    if !are_there_streams(streams) {
        println!("There is no available connections");
        return;
    }

    let mut streams = streams.lock().unwrap();

    for i in 0..streams.len() {
        let stream = streams.get(i).unwrap();
        println!("{}) - {}", i + 1, stream.peer_addr().unwrap());
    }
}

pub fn are_there_streams(streams: &Arc<Mutex<Vec<TcpStream>>>) -> bool {
    let mut streams = streams.lock().unwrap();
    !streams.is_empty()
}


pub fn get_stream_by_index(streams: &Arc<Mutex<Vec<TcpStream>>>, index: usize) -> &TcpStream {
    let mut streams = streams.lock().unwrap();
    streams.get(index).unwrap()
}