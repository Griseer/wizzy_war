// server/src/net/recv.rs

use crate::{ClientMessage, SocketAddr, UdpSocket};

pub fn recv_messages(socket: &UdpSocket) -> Vec<(SocketAddr, ClientMessage)> {
    let mut out = Vec::new();
    let mut buffer = [0u8; 2048];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((len, addr)) => {
                let data = &buffer[..len];

                if let Some(msg) = ClientMessage::decode(data) {
                    out.push((addr, msg));
                } else {
                    // mensaje inválido → se ignora
                    // opcional: log de debug
                    // eprintln!("Invalid packet from {addr}");
                }
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // no hay más paquetes
                break;
            }

            Err(_) => {
                // error real de socket
                break;
            }
        }
    }

    out
}
