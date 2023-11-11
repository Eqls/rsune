use std::io::{Read, Write};
use std::iter;
use std::net::{Shutdown, TcpListener, TcpStream};

mod read_buffer;

use read_buffer::ReadBuffer;

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
            println!("{:?}", &buffer[..n]);
            return match &buffer[..n][0] {
                14 => {
                    println!("{:?}", &buffer[..n]);
                    handle_login(stream)
                }
                15 => {
                    let update_keys = [
                        56, 79325, 55568, 46770, 24563, 299978, 44375, 0, 4176, 3589, 109125,
                        604031, 176138, 292288, 350498, 686783, 18008, 20836, 16339, 1244, 8142,
                        743, 119, 699632, 932831, 3931, 2974,
                    ];
                    let mut write_buffer = Vec::new();
                    write_buffer.push(0_u8);
                    for &key in update_keys.iter() {
                        // write_buffer.append(key.clone().to_be_bytes())
                        // let gg = ((key as u8) & 0xFF000000) >> 24;
                        write_buffer.push((((key as u32) & 0xFF000000) >> 24) as u8);
                        write_buffer.push((((key as u32) & 0x00FF0000) >> 16) as u8);
                        write_buffer.push((((key as u32) & 0x0000FF00) >> 8) as u8);
                        write_buffer.push(((key as u32) & 0x000000FF) as u8);
                        // write_buffer.push(key.clone());
                    }
                    println!("{:02X?}", &write_buffer);
                    println!("length: {}", &write_buffer.len());
                    stream.write(&write_buffer).unwrap();
                    write_buffer.clear();
                }
                n => println!("Other stuff {}", &n),
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

    let mut in_buffer = ReadBuffer::new(buffer[..len].to_vec());

    _ = in_buffer.read_bytes(64);
    let username = in_buffer.read_string().unwrap();
    println!("{:?}", username);
    let password = in_buffer.read_string().unwrap();
    println!("{:?}", password);

    out_buffer.push(2);
    out_buffer.append(&mut iter::repeat(0).take(2).collect::<Vec<u8>>());
    stream.write(&out_buffer).unwrap();
}
