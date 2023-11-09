use std::io::{Read, Write};
use std::iter;
use std::net::{Shutdown, TcpListener, TcpStream};

mod buffer;

use buffer::Buffer;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:43594").unwrap();
    println!("Server listening on port 43594");
    for stream in listener.incoming() {
        println!("called");
        match stream {
            Ok(stream) => {
                println!("{:?} connected!", stream.local_addr().unwrap());
                handle_connection(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    drop(listener);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 512 as usize];
    match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(n) => {
            println!("{:?}", &buffer[..n][0]);
            return match &buffer[..n][0] {
                14 => {
                    println!("{:?}", &buffer[..n]);
                    handle_login(stream)
                }
                _ => println!("Other stuff"),
            };
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

fn handle_login(mut stream: TcpStream) {
    let mut out_buffer = Vec::new();

    out_buffer.append(&mut iter::repeat(0).take(8).collect::<Vec<u8>>());

    // response code
    out_buffer.push(0_u8);

    // // server session key
    out_buffer.append(&mut 2_u64.to_le_bytes().to_vec());

    stream.write(&out_buffer).unwrap();
    out_buffer.clear();

    let mut buffer = vec![0; 512 as usize];
    let len = stream.read(&mut buffer).unwrap();

    let mut in_buffer = Buffer::new(buffer[..len].to_vec());

    _ = in_buffer.read_bytes(64);
    let username = in_buffer.read_string().unwrap();
    println!("{:?}", username);
    let password = in_buffer.read_string().unwrap();
    println!("{:?}", password);

    out_buffer.push(2);
    out_buffer.append(&mut iter::repeat(0).take(2).collect::<Vec<u8>>());
    stream.write(&out_buffer).unwrap();
}
