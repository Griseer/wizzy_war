// crates/client/src/main.rs
use net::{ClientMessage, ServerMessage};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:4000")?;

    let mut buffer = Vec::new();
    ClientMessage::Join.encode(&mut buffer);
    socket.send(&buffer)?;

    let mut recv_buf = [0u8; 1024];
    let len = socket.recv(&mut recv_buf)?;

    let msg = ServerMessage::decode(&recv_buf[..len]).unwrap();

    match msg {
        ServerMessage::Welcome { player_id } => {
            println!("Connected to server as Player {:?}", player_id);
        }
    }

    Ok(())
}
