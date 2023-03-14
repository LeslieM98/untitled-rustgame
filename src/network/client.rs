use crate::network::*;
use crate::GameServer;
use bevy_renet::renet::{ClientAuthentication, RenetClient, ServerAuthentication};
use bevy_renet::RenetClientPlugin;
use bincode::config;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

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

    pub fn get_socket_addr(&self) -> SocketAddr {
        match IpAddr::from_str(&self.ip) {
            Err(e) => panic!("{}", e),
            Ok(addr) => return SocketAddr::new(addr, self.port),
        }
    }

    fn build_renet(&self) -> RenetClient {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let addr = self.get_socket_addr();
        let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
        RenetClient::new(
            current_time,
            udp,
            Default::default(),
            ClientAuthentication::Unsecure {
                protocol_id: 0,
                client_id: 0,
                server_addr: addr,
                user_data: None,
            },
        )
        .unwrap()
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IpResource {
            value: self.ip.clone(),
        })
        .insert_resource(PortResource { value: self.port })
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(self.build_renet());
    }
}
