use std::net::TcpListener;

fn main() {
    println!("Hello there!");

    let tcpListener = TcpListener::bind("127.0.0.1:8081"); // try to bind to port 80

    for stream in tcpListener.unwrap().incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection successful!");
            },
            Err(stream) => {
                println!("Connection failed");
            }
        }
    }
}
