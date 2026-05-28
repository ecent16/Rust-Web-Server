use std::{
    net::{TcpListener, TcpStream}, 
    io::{BufReader, prelude::*}, 
};// Crate for tcp connections

fn main() {
    // bind acts as a new instance of a connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // incoming returns a sequence of streams
        let stream = stream.unwrap(); // unwrap will handle errors
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {

    let buf_reader = BufReader::new(&stream);

    // Collect all the tcp stream data
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|l| l.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
