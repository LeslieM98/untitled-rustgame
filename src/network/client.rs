use crate::network::lobby::LobbyClientPlugin;
use crate::network::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient};
use bevy_renet::RenetClientPlugin;
use std::net::{IpAddr, SocketAddr, UdpSocket};
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
                client_id: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs(),
                server_addr: addr,
                user_data: None,
            },
        )
        .unwrap()
    }
}

#[derive(Resource)]
pub struct ClientID {
    pub id: u64,
}

impl ClientID {
    pub fn new(id: u64) -> ClientID {
        ClientID { id }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let renet_client = self.build_renet();
        app.insert_resource(IpResource {
            value: self.ip.clone(),
        })
        .insert_resource(PortResource { value: self.port })
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(ClientID::new(renet_client.client_id()))
        .insert_resource(renet_client)
        .add_plugin(LobbyClientPlugin::default());
    }
}
