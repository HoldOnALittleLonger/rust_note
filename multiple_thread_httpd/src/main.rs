/* This program just the example from The Book.
 * All error handling just simply panics.
 */

use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*},
    thread,
    time::Duration,
    fs,
    string,
    vec
};

/* crate alias */
use multiple_thread_httpd as mthttpd;
use mthttpd::*;

/*
 * handle_http - function to handles http request
 * @connection:  tcp stream for current connection
 */
fn handle_http(mut connection: TcpStream) {
    let buffer: BufReader<_> = BufReader::new(&connection);
    let http_request: Vec<_> = buffer
        .lines()
        .map(|iter_line| iter_line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.len() == 0 {
        println!("received invalid request,ignore.");
        return;
    }

    println!("received http request:");
    println!("{http_request:#?}");

    const OK_ACCESS: &str = "hello.html";
    const BAD_ACCESS: &str = "404.html";

    let (status_line, filename) =
        match &(http_request[0])[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", OK_ACCESS),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(10));
                ("HTTP/1.1 200 OK", OK_ACCESS)
            },
            _ => ("HTTP/1.1 404 NOT FOUND", BAD_ACCESS),
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

    let thread_pool: ThreadPool = ThreadPool::new(4).unwrap();

    let tcp_listener = TcpListener::bind(IPv4_ENDPOINT).unwrap();
    println!("httpd started.");

    for iter_connection in tcp_listener.incoming().take(2) {
        let connection = iter_connection.unwrap();
        thread_pool.execute(|| handle_http(connection));
    }

    println!("shutting down.");
}
