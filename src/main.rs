mod http1;
mod request;


use std::io::{Read};
use request::Request;
use std::net::{TcpListener, TcpStream};


fn main() {
    // bind socket
    let socket = match TcpListener::bind("127.0.0.1:3000") {
        Ok(s) => {
            println!("Listening on 127.0.0.1:3000\n");
            s
        },
        Err(e) => panic!("Failed to bind socket: {:?}", e),
    };

    // handle connections
    for stream in socket.incoming() {
        let stream = stream.unwrap();
        on_connection(stream);
    }
}


fn on_connection(stream: TcpStream) {
    // determine if tls or http
    let mut is_http = false;
    let mut buffer = Vec::new();

    for byte in stream.try_clone().unwrap().bytes() {
        let byte = byte.unwrap();

        buffer.push(byte);

        if byte == 0xa {
            is_http = test_http(&buffer);
            break;
        }
    }

    let request = if is_http {
        // from http
        Request::from_http1(&mut buffer, stream.try_clone().unwrap())
    } else {
        // TODO - from tls
        Request::from_tls(&mut buffer, stream.try_clone().unwrap())
    };

    println!("{}", request);
}


fn test_http(buffer: &Vec<u8>) -> bool {
    let length = buffer.len();

    // too small for first http line
    if length < 10 {
        return false;
    }

    // `-2` t ignore \r and minor version of http
    let slice = &buffer[length-10..length-2];
    let test_slice = b"HTTP/1.";

    // test with slice of "HTTP/1."
    for i in 0..test_slice.len() {
        if slice[i] != test_slice[i] {
            return false;
        }
    }

    true
}
