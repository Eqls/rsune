mod buffer;
mod connection;
use connection::Connection;
use futures::FutureExt;
use futures::{SinkExt, StreamExt, TryFutureExt};
use std::env;
use tokio::{
    io::{AsyncReadExt, BufStream},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let listener = TcpListener::bind("127.0.0.1:43594").await.unwrap();
    println!("Server listening on port 43594");

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    connection.read_incomming_bytes().await.unwrap();
}
