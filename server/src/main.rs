use std::net::TcpListener; // Crate for tcp connections

fn main() {
    // bind acts as a new instance of a connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // incoming returns a sequence of streams
        let stream = stream.unwrap(); // unwrap will handle errors
        println!("Connection Established!");
    }
}
