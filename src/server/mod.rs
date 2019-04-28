mod parser;

use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::collections::VecDeque;
use std::cmp::min;

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


fn handle_get(number: u32, mut stream: TcpStream, mut data: VecDeque<String>) -> VecDeque<String> {
    (0..(min(number, data.len() as u32)))
        .fold(Vec::new(), |mut output_values, _| {
            output_values.push(data.pop_front());
            output_values.push(Some(" ".to_string()));
            output_values
        }).iter()
        .for_each(|value| {
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
    // TODO
}


#[test]
fn test_handle_pub() {
    // TODO
}


#[test]
fn test_handle_get() {
    // TODO
}


#[test]
fn test_handle_error() {
    // TODO
}


#[test]
fn test_serve() {
    // TODO
}
