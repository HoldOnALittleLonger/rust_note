use std::net::{TcpListener, TcpStream};
use std::io::*;
use std::string;
use std::vec;
use std::fs;

fn handle_http(mut connection: TcpStream) {
    let buffer: BufReader<_> = BufReader::new(&connection);
    let http_request: Vec<_> = buffer
        .lines()
        .map(|iter_line| iter_line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("received http request:");
    println!("{http_request:#?}");

    const OK_ACCESS: &str = "hello.html";
    const BAD_ACCESS: &str = "404.html";

    let (status_line, filename) =
        if http_request[0] == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", OK_ACCESS)
        } else {
            ("HTTP/1.1 404 NOT FOUND", BAD_ACCESS)
        };

    let contents = fs::read_to_string(filename).unwrap();
    let contents_length = contents.len();
    let http_respone = 
        format!("{status_line}\r\nContent-Length: \
                 {contents_length}\r\n\r\n{contents}");        
    /* '\' is used to tell rust do not include newline in the string */
    connection.write_all(http_respone.as_bytes()).unwrap();
}

fn main() {
    const IPv4_ENDPOINT: &str = "127.0.0.1:8080";
    println!("httpd starting at {} .", IPv4_ENDPOINT);

    let tcp_listener = TcpListener::bind(IPv4_ENDPOINT).unwrap();
    println!("httpd started.");

    for iter_connection in tcp_listener.incoming() {
        let connection = iter_connection.unwrap();
        handle_http(connection);
    }
}
