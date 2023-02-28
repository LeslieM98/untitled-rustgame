use crate::network::*;
use std::net::UdpSocket;

#[derive(Resource)]
pub struct ListeningServer {
    value: UdpSocket,
}

impl ListeningServer {
    fn bind(ip: String, port: u32) -> Self {
        let bind_address = format!("{}:{}", ip, port);
        let socket = match UdpSocket::bind(bind_address.clone()) {
            Ok(socket) => Self { value: socket },
            Err(error) => panic!("{}", error.to_string()),
        };

        info!("Server opened at {}", bind_address);
        socket
    }
    pub fn get(&self) -> &UdpSocket {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut UdpSocket {
        &mut self.value
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
        .insert_resource(ListeningServer::bind(self.ip.clone(), self.port));
    }
}
