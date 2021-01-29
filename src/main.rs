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
    while match stream.read(&mut buffer) {
        Ok(0) => false,
        Ok(n) => {
            // println!("{:?}", &buffer[..n][0]);
            match &buffer[..n][0] {
                14 => {
                    println!("Initiliazing a new connection.")
                }
                _ => println!("Other stuff"),
            }
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
