use std::env;
use std::net::TcpListener;

use common::handle_connection;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::from("");
    let mode = args.iter().nth(1).unwrap_or(&default);
    let mode = if mode == "container" { "container" } else { "local" };

    println!("pong is running ({})... listening for pings...", mode);

    let localhost = if mode == "local" { "127.0.0.1:8787" } else { "0.0.0.0:8787" };
    let listener = TcpListener::bind(localhost).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, "ping", "pong", mode)
    }
}