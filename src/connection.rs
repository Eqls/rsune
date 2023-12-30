use crate::buffer::RS2BufExt;
use bytes::{Buf, BufMut, BytesMut};
use futures::FutureExt;
use std::io::{self, Cursor};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

pub enum LoginState {
    Handshake,
    Header,
    Payload(u8, usize),
}

pub enum ConnectionState {
    Handshake,
    Login(LoginState),
    Play,
}

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
    state: ConnectionState,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
            state: ConnectionState::Handshake,
        }
    }

    pub async fn read_incomming_bytes(&mut self) -> tokio::io::Result<()> {
        loop {
            let mut buf = Cursor::new(&self.buffer);
            println!("{:?}", &buf);
            match &mut self.state {
                ConnectionState::Handshake => {
                    if buf.has_remaining() {
                        let id = buf.read_u8().await;
                        println!("{:?}", id);
                        self.buffer.advance(1);
                        self.state = ConnectionState::Login(LoginState::Handshake);
                        continue;
                    }
                }
                ConnectionState::Login(login_state) => {
                    match login_state {
                        LoginState::Handshake => {
                            if buf.has_remaining() {
                                let username_hash = buf.get_u8();
                                println!("Username hash: {:?}", &username_hash);
                                println!("Buff length: {:?}", buf.remaining());
                                println!("time to login now!");
                                let mut out_buffer = BytesMut::with_capacity(17);

                                out_buffer.put_bytes(0, 8);

                                out_buffer.put_u8(0);

                                // Server seed
                                out_buffer.put_u64(0);

                                self.stream.write_all(&out_buffer).await;
                                self.stream.flush().await;
                                println!("{:?}", out_buffer);
                                self.buffer.advance(buf.position() as usize);
                                *login_state = LoginState::Header;
                                continue;
                            }
                        }
                        LoginState::Header => {
                            println!("header stage");
                            if buf.remaining() >= 2 {
                                let login_type = buf.get_u8();
                                println!("Login type: {:?}", &login_type);

                                let login_len = buf.get_u8();
                                println!("Login length: {:?}", &login_len);
                                self.buffer.advance(buf.position() as usize);
                                *login_state = LoginState::Payload(login_type, login_len as usize);
                                continue;
                            }
                        }
                        LoginState::Payload(login_type, login_len) => {
                            println!("payload stage");
                            if buf.remaining() >= *login_len {
                                let version = buf.get_u8();
                                println!("Client version: {:?}", &version);

                                let revision = buf.get_u16();
                                println!("Revision: {:?}", &revision);

                                let memory_status = buf.get_u8();
                                println!("Memory status: {:?}", &memory_status);

                                let mut crcs: [i32; 9] = Default::default();

                                for i in 0..9 {
                                    crcs[i] = buf.get_i32();
                                }

                                println!("Archives: {:?}", &crcs);

                                //will be reletvant when RSA encryption will be enabled.
                                //let packet_len = buf.get_u8();
                                //println!("Packet len: {:?}", &packet_len);

                                let id = buf.get_u8();
                                println!("Id: {:?}", &id);

                                let client_seed = buf.get_u64();
                                println!("Client seed: {:?}", &client_seed);

                                let reported_seed = buf.get_u64();
                                println!("Reported seed: {:?}", &reported_seed);

                                let uuid = buf.get_i32();
                                println!("UUID: {:?}", &uuid);

                                let username = buf.get_string();
                                println!("Username: {:?}", &string);

                                let password = buf.get_string();
                                println!("Password: {:?}", &pasword);
                            }
                        }
                    }
                }
                ConnectionState::Play => todo!(),
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    println!("no bytes")
                    // return Ok(());
                } else {
                    todo!("Handle connection reset")
                    // return Err();
                }
            }
        }
    }
}
