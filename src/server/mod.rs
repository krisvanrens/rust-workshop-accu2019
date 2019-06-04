mod parser;

use std::cmp::min;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::io;
use std::net::{TcpListener, TcpStream};

#[cfg(test)]
use std::net::{Shutdown};

use parser::{Command, ParseError, parse_error_to_string, parse};


fn handle_client(mut stream: TcpStream) -> Option<(String, TcpStream)> {
    let mut data = String::new();
    let mut buffer = [0];

    loop {
        stream.read_exact(&mut buffer).ok()?;
        if buffer[0] == '\n' as u8 {
            break;
        }

        data.push(buffer[0] as char);
    }

    Some((data, stream))
}


fn handle_pub(values: Vec<String>, mut data: VecDeque<String>) -> VecDeque<String> {
    values.into_iter().for_each(|x|data.push_back(x));
    data
}


// TODO: Rewrite handle_get to not consume the stream.
fn handle_get(number: u32, mut stream: TcpStream, mut data: VecDeque<String>) -> VecDeque<String> {
    let mut output = (0..(min(number, data.len() as u32)))
        .fold(Vec::new(), |mut values, _| {
            values.push(data.pop_front());
            values.push(Some(" ".to_string()));
            values
        });

    // Remove trailing space item.
    if output.len() > 1 {
        output.remove(output.len() - 1);
    }

    output.iter().for_each(|value| {
        match stream.write(value.as_ref().unwrap().as_bytes()) {
            Err(error) => println!("Error: {}", error),
            _ => ()
        }
    });

    data
}


fn handle_error(error: ParseError, mut stream: TcpStream) {
    match stream.write(("Error: ".to_string() + &parse_error_to_string(error)).as_bytes()) {
        Err(error) => println!("Error: {}", error),
        _ => ()
    }
}


pub fn serve(ip: &str, port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(ip.to_string() + ":" + &port.to_string())?;
    let mut data : VecDeque<String> = VecDeque::new();

    for stream in listener.incoming() {
        let (result, stream) = handle_client(stream.unwrap()).unwrap();
        match parse(&result) {
            Ok(Command::Pub(values)) => data = handle_pub(values, data),
            Ok(Command::Get(number)) => data = handle_get(number, stream, data),
            Err(error) => handle_error(error, stream)
        }

        println!("Current buffer state: {:?}", data);
    }

    Ok(())
}


#[test]
fn test_handle_client() {
    // Create a TcpStream and shut it down for to make it invalid for handle_client.
    let socket_address = "127.0.0.1:12345";
    let _listener = TcpListener::bind(socket_address);
    let stream = TcpStream::connect(socket_address).unwrap();
    assert_eq!(stream.shutdown(Shutdown::Both).is_ok(), true);

    assert_eq!(handle_client(stream).is_none(), true);
}


#[test]
fn test_handle_pub() {
    let mut data : VecDeque<String> = VecDeque::new();

    data = handle_pub(vec![], data);
    assert!(data.is_empty());

    data = handle_pub(vec!["one".to_string(), "two".to_string()], data);
    assert_eq!(data, ["one", "two"]);
}


#[test]
fn test_handle_get() {
    // TODO: Not a comprehensive test, add more tests when handle_get does not consume stream.

    let socket_address = "127.0.0.1:12345";
    let _listener = TcpListener::bind(socket_address);
    let stream = TcpStream::connect(socket_address).unwrap();

    let mut data : VecDeque<String> = VecDeque::new();
    data.push_back("one".to_string());
    data.push_back("two".to_string());

    data = handle_get(2, stream, data);
    assert_eq!(data.len(), 0);
}


#[test]
fn test_handle_error() {
    let socket_address = "127.0.0.1:12345";
    let _listener = TcpListener::bind(socket_address);
    let stream = TcpStream::connect(socket_address).unwrap();

    handle_error(ParseError::InvalidCommand, stream);
}


#[test]
fn test_serve() {
    // TODO: Not a comprehensive test, extend.

    assert_eq!(serve("nonsense", 12345).is_err(), true);

    // Occupy port 12345 so the test server can't.
    let socket_address = "127.0.0.1:12345";
    let _listener = TcpListener::bind(socket_address);

    assert_eq!(serve("127.0.0.1", 12345).is_err(), true);
    assert_eq!(serve("nonsense",  12345).is_err(), true);
}
