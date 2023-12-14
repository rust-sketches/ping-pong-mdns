use std::{io, thread, time};
use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn parse_http_request(reader: &mut impl io::BufRead) -> Result<(String, HashMap<String, String>, Option<String>), io::Error> {
    let mut request = String::new();
    reader.read_line(&mut request)?;

    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(size) if size > 2 => { // a blank line (CRLF only) separates HTTP headers and body
                match line.split_once(": ") { // HTTP headers are always formatted as "key: value"
                    Some((key, value)) => headers.insert(key.trim().into(), value.trim().into()),
                    None => continue, // skip any header lines that can't be parsed
                };
            },
            _ => break // if the reader fails to read the next line, quit early
        };
    }

    let mut body: Option<String> = None;

    if let Some(length) = headers.get("Content-Length") {
        if let Ok(length) = length.parse::<usize>() {
            let mut buffer = vec![0; length];
            reader.read_exact(&mut buffer).unwrap();
            body = Some(std::str::from_utf8(buffer.as_slice()).unwrap().into());
        }
    }

    Ok((request.trim().into(), headers, body))
}

pub fn respond(response: Result<&str, &str>, stream: &mut TcpStream, mode: &str) {
    let (status, msg) = match response {
        Ok(msg) => ("HTTP/1.1 200 OK", msg),
        Err(msg) => ("HTTP/1.1 404 NOT FOUND", msg)
    };

    thread::sleep(time::Duration::from_secs(1));

    let ack = format!("{status}\r\n\r\n");
    stream.write_all(ack.as_bytes()).unwrap();

    match response {
        Ok(endpoint) => {
            let port = if endpoint == "ping" { 8787 } else { 7878 };
            let ip = if mode == "local" { "127.0.0.1" } else { "172.17.0.1" };
            let host = format!("{}:{}", ip, port);

            let address = host.to_socket_addrs().unwrap().next().unwrap();
            let timeout = Duration::from_millis(5000);
            let mut socket = TcpStream::connect_timeout(&address, timeout).unwrap();

            let payload = format!("POST /{} HTTP/1.1\r\nHost: {}", endpoint, host);

            socket.write(payload.as_bytes()).unwrap();
            socket.flush().unwrap();
        }
        _ => ()
    }
}

pub fn handle_connection(mut stream: TcpStream, listen_for: &str, send: &str, mode: &str) {
    let mut reader = BufReader::new(&mut stream);

    match parse_http_request((&mut reader).into()) {
        Ok((request, headers, body)) => {
            if request == format!("POST /{} HTTP/1.1", listen_for) {
                respond(
                    Ok(send),
                    &mut stream,
                    mode
                );

                println!("received {}, sending {}", listen_for, send);

            } else {
                respond(
                    Err("unrecognized request received"),
                    &mut stream,
                    mode
                );

                println!("received unexpected request: {}", request);
            }
        },
        Err(_) => println!("Could not parse http request")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_http_request_no_body() {
        let mut request = "\
POST /pong HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/8.1.2
Accept: */*
        ";

        let (request, headers, body) = parse_http_request(&mut request.as_bytes()).unwrap();

        assert_eq!(request, "POST /pong HTTP/1.1");

        assert_eq!(headers.get("Host").unwrap(),       "127.0.0.1:7878");
        assert_eq!(headers.get("User-Agent").unwrap(), "curl/8.1.2");
        assert_eq!(headers.get("Accept").unwrap(),     "*/*");

        assert_eq!(body, None);
    }

    #[test]
    fn test_parse_http_request_with_body() {
        let mut request = "\
POST /pong HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/8.1.2
Accept: */*
Content-Length: 13
Content-Type: application/x-www-form-urlencoded

Hello, World!
        ";

        let (request, headers, body) = parse_http_request(&mut request.as_bytes()).unwrap();

        assert_eq!(request, "POST /pong HTTP/1.1");

        assert_eq!(headers.get("Host").unwrap(),           "127.0.0.1:7878");
        assert_eq!(headers.get("User-Agent").unwrap(),     "curl/8.1.2");
        assert_eq!(headers.get("Accept").unwrap(),         "*/*");
        assert_eq!(headers.get("Content-Length").unwrap(), "13");
        assert_eq!(headers.get("Content-Type").unwrap(),   "application/x-www-form-urlencoded");

        assert_eq!(body.unwrap(), "Hello, World!");
    }

}