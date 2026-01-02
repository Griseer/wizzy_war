// crates/client/src/main.rs
use bevy::prelude::*;
use std::net::UdpSocket;

//mod input;
mod net;
//mod render;
//mod world;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:4000")?;
    socket.set_nonblocking(true)?;

    net::send::send_join(&socket)?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            net::NetPlugin::new(socket),
            //world::WorldPlugin,
            //render::RenderPlugin,
            //input::InputPlugin,
        ))
        .run();

    Ok(())
}
