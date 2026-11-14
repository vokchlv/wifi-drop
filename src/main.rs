use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{env, fs};

fn main() {
    let file_path = get_file_path_from_arguments().unwrap();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &file_path);

        println!("Connection established!");
    }
}

fn get_file_path_from_arguments() -> Option<String> {
    for argument in env::args() {
        if argument.starts_with("target") {
            continue;
        }
        if Path::new(&argument).exists() {
            return Some(argument);
        }
    }
    None
}

fn handle_connection(mut stream: TcpStream, file_path: &String) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
