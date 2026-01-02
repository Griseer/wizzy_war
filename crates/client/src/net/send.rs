// client/src/net/send.rs

use crate::net::ClientMessage;
use std::net::UdpSocket;

pub fn send_join(socket: &UdpSocket) -> std::io::Result<()> {
    let mut buf = Vec::new();
    ClientMessage::Join.encode(&mut buf);
    socket.send(&buf)?;
    Ok(())
}

pub fn send_input(socket: &UdpSocket, msg: &ClientMessage) {
    let mut buf = Vec::new();
    msg.encode(&mut buf);
    let _ = socket.send(&buf);
}
