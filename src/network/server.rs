use crate::network::*;
use std::net::{SocketAddr, TcpListener};

pub enum ConnectionPacket {
    Initiate,
    ConnectionGranted {
        udp_target: SocketAddr,
        player_identifier: usize,
    },
    ConnectionRefused(usize),
}

#[derive(Resource)]
pub struct ConnectionServer {
    value: TcpListener,
}

impl ConnectionServer {
    fn new(ip: &str, port: u32) -> Self {
        let ip_str = format!("{}:{}", ip, port);
        let socket = ConnectionServer {
            value: TcpListener::bind(&ip_str).expect("Cannot open Connection Server"),
        };
        info!("Opening Server at {}", ip_str);
        socket
    }
}

pub struct ServerPlugin {
    ip: String,
    port: u32,
}

impl ServerPlugin {
    pub fn new(ip: &str, port: u32) -> Self {
        Self {
            ip: String::from(ip),
            port,
        }
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IpResource {
            value: self.ip.clone(),
        })
        .insert_resource(PortResource { value: self.port })
        .insert_resource(ConnectionServer::new(&self.ip, self.port));
    }
}

fn handle_incoming_connections(connection_server: Res<ConnectionServer>) {}
