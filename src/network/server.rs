use crate::actor;
use crate::actor::player::{PlayerBundle, PlayerMarker};
use crate::actor::*;
use bevy_renet::renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use bevy_renet::RenetServerPlugin;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::time::SystemTime;

use crate::network::*;

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
        ServerConfig::new(5, 0, self.get_socket_addr(), ServerAuthentication::Unsecure)
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
        .add_plugin(RenetServerPlugin::default())
        .add_system(handle_events_system);
    }
}

fn handle_events_system(
    mut commands: Commands,
    mut server_events: EventReader<ServerEvent>,
    players: Query<(Entity, &actor::Name), With<PlayerMarker>>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _user_data) => {
                commands.spawn(
                    PlayerBundle::default()
                        .with_name(actor::Name::new(format!("Player_{}", id).into())),
                );
                println!("Client {} connected", id);
            }
            ServerEvent::ClientDisconnected(id) => {
                for (entity, name) in &players {
                    if name.value.ends_with(&format!("_{}", id)) {
                        commands.entity(entity).despawn()
                    }
                }
                println!("Client {} disconnected", id);
            }
        }
    }
}
