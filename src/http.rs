use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::Path,
};

fn define_http_request(mut stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}

pub fn handle_input_connection(mut stream: TcpStream) {
    let http_request = define_http_request(&stream);

    // ignore secondary browser requests
    if http_request.first().map(String::as_str) != Some("GET / HTTP/1.1") {
        stream
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
            .unwrap();
        return;
    }
}

pub fn handle_output_connection(mut stream: TcpStream, file_path: &String) {
    let http_request = define_http_request(&stream);

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
