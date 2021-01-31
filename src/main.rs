use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::thread::JoinHandle;
use std::{
    collections::HashMap,
    io::{Read, Write},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:43595").unwrap();
    println!("Server listening on port 43595");
    for stream in listener.incoming() {
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
            // println!("{:?}", &buffer[..n][0]);
            return match &buffer[..n][0] {
                14 => handle_login(stream),
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
    {}
}

fn handle_login(mut stream: TcpStream) {
    let mut buffer = vec![0; 512 as usize];
    match stream.read(&mut buffer) {
        Ok(n) => println!("{:?}", &buffer[..n]),
        Err(err) => {
            println!("An error occurred", err);
        }
    }

    for _ in 0..7 {
        stream.write(&0_i8.to_le_bytes()).unwrap();
    }
    stream.write(&2_u32.to_le_bytes()).unwrap();
}
