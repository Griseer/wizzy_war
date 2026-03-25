// server/src/main.rs
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use ::net::client::message::ClientMessage;
use shared::ids::PlayerId;

use crate::net::send::broadcast_snapshot;
use crate::tick::run_tick;

//mod input;
mod net;
mod tick;
mod state;


const SERVER_TICK_RATE: u64 = 60;
const TICK_DURATION: Duration = Duration::from_millis(1_000 / SERVER_TICK_RATE);
const DT:f32 = 1.0 / SERVER_TICK_RATE as f32;

fn main() -> std::io::Result<()> {
    // -------------------------
    // Socket
    // -------------------------

    let socket = UdpSocket::bind("0.0.0.0:4000")?;
    socket.set_nonblocking(true)?;
    println!("Server listening on 0.0.0.0:4000");

    //// -------------------------
    //// State
    //// -------------------------
    let mut state_server = state::ServerState::new(SERVER_TICK_RATE);
    let mut next_tick = Instant::now();
    const MAX_TICKS_PROCESSED: u8 = 5;

    // -------------------------
    // Main loop
    // -------------------------
    loop {
        // 1️⃣ Receive network messages
        for (addr, msg) in net::recv::recv_messages(&socket) {
            match msg {
                ClientMessage::Join => {
                    let player_id = state_server.add_player(addr);
                    net::send::send_welcome(&socket, addr, player_id);
                }
                ClientMessage::Input(input_msg) => {
                    state_server.handle_input(addr, input_msg);
                }
            }
        }

        // 2️⃣ Fixed tick

        let mut ticks_processed: u8 = 0;
        while Instant::now() >= next_tick && ticks_processed < MAX_TICKS_PROCESSED {
            run_tick(&mut state_server,  DT);
            broadcast_snapshot(&socket, &state_server);

            next_tick += TICK_DURATION;
            ticks_processed += 1;
        }

        let now = Instant::now();
        if next_tick > now {
            std::thread::sleep(next_tick - now)
        };
    }
}
