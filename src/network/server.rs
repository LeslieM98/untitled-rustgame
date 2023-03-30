use crate::actor;
use crate::actor::player::{PlayerBundle, PlayerMarker};
use crate::actor::*;
use bevy::utils::{HashMap, HashSet};
use bevy_renet::renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use bevy_renet::RenetServerPlugin;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::time::SystemTime;

use crate::network::*;

pub const MAX_CONNECTIONS: usize = 5;

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
        .insert_resource(Lobby::default())
        .add_plugin(RenetServerPlugin::default())
        .add_system(handle_events_system);
    }
}

#[derive(Resource, Default)]
struct Lobby {
    ids: HashMap<u64, Entity>,
}

#[derive(Component)]
struct ClientID {
    id: u64,
}

fn handle_events_system(
    mut commands: Commands,
    mut server_events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _user_data) => {
                let entity = commands
                    .spawn(
                        PlayerBundle::default()
                            .with_name(actor::Name::new(format!("Player_{}", id).into())),
                    )
                    .insert(ClientID { id: *id })
                    .id();
                lobby.ids.insert(*id, entity);

                println!("Client {} connected", id);
            }
            ServerEvent::ClientDisconnected(id) => {
                let entity = lobby.ids.remove(id).expect("Client not connected");
                commands.entity(entity).despawn();
                println!("Client {} disconnected", id);
            }
        }
    }
}
