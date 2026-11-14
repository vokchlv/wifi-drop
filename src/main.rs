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

    // ignore secondary browser requests
    if http_request.first().map(String::as_str) != Some("GET / HTTP/1.1") {
        stream
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
            .unwrap();
        return;
    }

    // read raw bytes
    let contents = fs::read(file_path).unwrap();
    let length = contents.len();

    // extrect file name from path
    let filename = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download");

    // content-disposition to force browser to save
    let headers = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: application/octet-stream\r\n\
        Content-Disposition: attachment; filename=\"{filename}\"\r\n\
        Content-Length: {length}\r\n\r\n"
    );

    stream.write_all(headers.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
}
