use std::net::SocketAddr;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpStream}};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Client {:?} connected", addr);
        
        tokio::spawn(handle_connection(socket, addr));
    }

}

async fn handle_connection(mut socket: TcpStream, addr: SocketAddr) {
    let mut buffer= [0; 1024];
    let bytes_read = socket.read(&mut buffer).await.unwrap();

    if bytes_read == 0 {
        println!("Client {:?} disconnected", addr);
        return;
    }

    socket.write_all(&buffer[0..bytes_read]).await.unwrap();
}