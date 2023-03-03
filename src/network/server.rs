use crate::network::server::ConnectionPacket::{ConnectionGranted, ConnectionRefused};
use crate::network::*;
use bevy::utils::HashMap;
use bincode::error::DecodeError;
use bincode::{config, Decode, Encode};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, UdpSocket};

pub struct ServerPlugin {
    ip: String,
    port: u16,
}

impl ServerPlugin {
    pub fn new(ip: &str, port: u16) -> Self {
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
        .insert_resource(ConnectionServer::new(&self.ip, self.port))
        .insert_resource(Connections::default())
        .add_system(handle_incoming_connections_system);
    }
}

#[derive(Decode, Encode, Debug)]
pub enum ConnectionPacket {
    Initiate(SocketAddr),
    ConnectionGranted(SocketAddr, PlayerIdentifier),
    ConnectionRefused(usize),
}

#[derive(Resource)]
pub struct ConnectionServer {
    listener: TcpListener,
}

impl ConnectionServer {
    pub fn new(ip: &str, port: u16) -> Self {
        let ip_str = format!("{}:{}", ip, port);
        let socket = ConnectionServer {
            listener: TcpListener::bind(&ip_str).expect("Cannot open Connection Server"),
        };
        info!("Opening Server at {}", ip_str);
        socket
    }

    pub fn handle_connection_protocol(&self, connections: &mut Connections) {
        // server answers with udp port
        for incoming in self.listener.incoming() {
            let mut stream = incoming.expect("Cannot open Tcp Stream");
            let mut read_buf = [0; 512];
            // client sends -> server
            let read_size = stream.read(&mut read_buf).expect("Network errors");
            let (read_packet, read_packet_size): (ConnectionPacket, usize) =
                bincode::decode_from_slice(&read_buf[0..read_size], config::standard())
                    .expect("Decoding error");
            let answer_package = if let ConnectionPacket::Initiate(client_address) = read_packet {
                let client = UdpSocket::bind("localhost:0").expect("Cannot open UDP socket");
                client
                    .connect(client_address)
                    .expect("Cannot connect to client");
                let server_addr = client.local_addr().expect("Cannot get local address");
                let player_id = connections
                    .add_connection(client)
                    .expect("No player id free");

                info!("{} connected", server_addr);

                ConnectionGranted(server_addr, player_id)
            } else {
                ConnectionRefused(0)
            };
            let encoded_answer = bincode::encode_to_vec(answer_package, config::standard())
                .expect("Error encoding answer packet");
            stream.write(&encoded_answer).expect("Error sending packet");
        }
    }
}

#[derive(Resource, Default)]
pub struct Connections {
    value: HashMap<PlayerIdentifier, UdpSocket>,
}

impl Connections {
    pub fn add_connection(&mut self, connection: UdpSocket) -> Option<PlayerIdentifier> {
        for new_id in 0..256 {
            if !self.value.contains_key(&new_id) {
                self.value.insert(new_id, connection);
                return Some(new_id);
            }
        }
        None
    }
}

pub fn handle_incoming_connections_system(
    mut active_connections: ResMut<Connections>,
    connection_server: Res<ConnectionServer>,
) {
    handle_incoming_connections(&mut active_connections, &connection_server);
}

pub fn handle_incoming_connections(
    active_connections: &mut Connections,
    connection_server: &ConnectionServer,
) {
    connection_server.handle_connection_protocol(active_connections);
}
