use std::env;
use std::net::TcpListener;

use common::handle_connection;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::from("");
    let mode = args.iter().nth(1).unwrap_or(&default);
    let mode = if mode == "container" { "container" } else { "local" };

    println!("ping is running ({})... listening for pongs...", mode);

    let localhost = if mode == "local" { "127.0.0.1:7878" } else { "0.0.0.0:7878" };
    let listener = TcpListener::bind(localhost).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, "pong", "ping", mode)
    }
}