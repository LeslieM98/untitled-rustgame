use bevy_renet::renet::{RenetServer, ServerAuthentication, ServerConfig};
use std::net::{IpAddr, SocketAddr};
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
        let ip_addr = IpAddr::from_str(&self.ip).expect("Wrong IP");

        SocketAddr::new(ip_addr, self.port)
    }

    pub fn get_server_config(&self) -> ServerConfig {
        ServerConfig::new(5, 0, self.get_socket_addr(), ServerAuthentication::Unsecure)
    }

    pub fn create_server(&self) -> RenetServer {
        let config = self.get_server_config();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        RenetServer::new(current_time, config, Default::default(), ()).expect("RenetError")
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IpResource {
            value: self.ip.clone(),
        })
        .insert_resource(PortResource { value: self.port });
    }
}
