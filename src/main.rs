#![allow(non_snake_case)]
use std::{io::{BufRead, Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    println!("Hello there!");
    println!("Started http server, listening on port: 127.0.0.1:8081...");

    let tcp_listener = TcpListener::bind("127.0.0.1:8081"); // try to bind to port 80

    for stream in tcp_listener.unwrap().incoming() {
        match stream {
            Ok(stream) => {
                client_handle(stream);
            },
            Err(stream) => {
                println!("Connection failed error: {}", stream);
            }
        }
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let http_request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    stream.flush().unwrap();

}

fn handle_response(mut stream: TcpStream) {
    let response = b"HTTP/1.0 200 OK\r
            Content-Type: text/html; charset=UTF-8\r
            Content-Length: 37\r
            \r\n\r\n
            <!DOCTYPE html>
            <html><head><body>Hello world!!!</head></body></html>\r";
    match stream.write_all(response) {
        Ok(_) => println!("Succesfuly sent message"),
        Err(ex) => println!("Failed {}", ex),
    }
    stream.flush().unwrap();
}

fn client_handle(stream: TcpStream) {
    handle_connection(&stream);
    handle_response(stream);
}