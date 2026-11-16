mod cli;
mod http;
mod network;
mod ui;

fn main() {
    let Some(file_path) = cli::get_file_path_from_arguments() else {
        eprintln!("No file path was specified; exit the program.");
        return;
    };

    let server = network::create_tcp_listener();
    ui::print_qr_code(&server.url());
    println!("Listening on: {}", server.ip);

    for stream in server.listener.incoming() {
        let stream = stream.unwrap();
        http::handle_output_connection(stream, &file_path);
        println!("Connection established!");
    }
}
