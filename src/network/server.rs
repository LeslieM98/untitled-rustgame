use bevy_renet::renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use bevy_renet::RenetServerPlugin;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::time::SystemTime;

use crate::network::lobby::LobbyServerPlugin;
use crate::network::remote_player::ServerPlayerSyncPlugin;
use crate::network::*;

pub const MAX_CONNECTIONS: usize = 5;

pub struct ClientConnectedEvent {
    pub id: u64,
}

pub struct ClientDisconnectedEvent {
    pub id: u64,
}

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

    pub fn get_socket_addr(&self) -> SocketAddr {
        match IpAddr::from_str(&self.ip) {
            Err(e) => panic!("{}", e),
            Ok(addr) => return SocketAddr::new(addr, self.port),
        }
    }

    pub fn get_server_config(&self) -> ServerConfig {
        ServerConfig::new(
            MAX_CONNECTIONS,
            0,
            self.get_socket_addr(),
            ServerAuthentication::Unsecure,
        )
    }

    pub fn create_server(&self) -> RenetServer {
        let config = self.get_server_config();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let udp = UdpSocket::bind(self.get_socket_addr()).expect("Cannot create udp socket");
        RenetServer::new(current_time, config, Default::default(), udp).expect("Renet Error")
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IpResource {
            value: self.ip.clone(),
        })
        .insert_resource(PortResource { value: self.port })
        .insert_resource(self.create_server())
        .add_event::<ClientConnectedEvent>()
        .add_event::<ClientDisconnectedEvent>()
        .add_plugin(RenetServerPlugin::default())
        .add_plugin(LobbyServerPlugin::default())
        .add_plugin(ServerPlayerSyncPlugin)
        .add_system(handle_events_system);
    }
}

fn handle_events_system(
    mut server_events: EventReader<ServerEvent>,
    mut client_connected_events: EventWriter<ClientConnectedEvent>,
    mut client_disconnected_events: EventWriter<ClientDisconnectedEvent>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _user_data) => {
                info!("Client connected");
                client_connected_events.send(ClientConnectedEvent { id: *id });
            }
            ServerEvent::ClientDisconnected(id) => {
                client_disconnected_events.send(ClientDisconnectedEvent { id: *id })
            }
        }
    }
}
