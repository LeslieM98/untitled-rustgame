use crate::network::*;
use bevy::prelude::*;
use std::net::{IpAddr, TcpListener, TcpStream, UdpSocket};

pub struct ClientPlugin {
    ip: String,
    port: u16,
}

impl ClientPlugin {
    pub fn new(ip: &str, port: u16) -> Self {
        Self {
            ip: String::from(ip),
            port,
        }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IpResource {
            value: self.ip.clone(),
        })
        .insert_resource(PortResource { value: self.port })
        .insert_resource(ConnectionServer::new(self.ip.clone(), self.port));
    }
}

#[derive(Resource)]
struct ConnectionServer {
    remote_server: TcpStream,
}

impl ConnectionServer {
    fn new(ip: String, port: u16) -> Self {
        let socket =
            TcpStream::connect(format!("{}:{}", ip, port)).expect("Cannot create TCP stream");
        Self {
            remote_server: socket,
        }
    }
}

#[derive(Resource)]
struct GameConnection {
    socket: UdpSocket,
}

impl GameConnection {
    fn new() -> Self {
        let socket = UdpSocket::bind(format!("localhost:0")).expect("Cannot open UDP socket");
        Self { socket }
    }
}
