use crate::network::server::ConnectionPacket;
use crate::network::server::ConnectionPacket::{ConnectionRefused, Initiate};
use crate::network::*;
use crate::GameServer;
use bincode::config;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};

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

    fn initiate_connection(
        &mut self,
        local_address: SocketAddr,
    ) -> Result<(SocketAddr, PlayerIdentifier), String> {
        let write_packet = Initiate(local_address);
        let write_bytes =
            bincode::encode_to_vec(write_packet, config::standard()).expect("Encoding error");
        let _written_size = self
            .remote_server
            .write(&write_bytes)
            .expect("Cannot send message");

        let mut answer_buffer = [0; 1024];
        let answer_size = self
            .remote_server
            .read(&mut answer_buffer)
            .expect("Cannot read server answer");
        let (answer, _decoded_size): (ConnectionPacket, usize) =
            bincode::decode_from_slice(&answer_buffer[0..answer_size], config::standard())
                .expect("Cannot decode packet");
        match answer {
            ConnectionPacket::ConnectionGranted(address, player_id) => Ok((address, player_id)),
            ConnectionRefused(error_code) => Err(format!("{:?}", ConnectionRefused(error_code))),
            _ => Err(String::from("Unknown network error")),
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
    fn connect(&mut self, address: SocketAddr) {
        self.socket
            .connect(address)
            .expect("Cannot connect to game server")
    }
}

#[derive(Resource)]
pub struct PlayerId {
    player_id: PlayerIdentifier,
}

fn connect_to_serer(
    mut connection_server: ResMut<ConnectionServer>,
    mut game_server: ResMut<GameConnection>,
    mut player_id_res: ResMut<PlayerId>,
) {
    let local_addr = game_server
        .socket
        .local_addr()
        .expect("Cannot fetch local addr");

    let (game_server_addr, player_id) = connection_server
        .initiate_connection(local_addr)
        .expect("Cannot connect to game server");

    game_server.connect(game_server_addr);
    player_id_res.player_id = player_id;
}
