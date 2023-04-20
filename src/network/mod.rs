use bevy::prelude::*;

pub mod client;
pub mod lobby;
pub mod packet_communication;
pub mod remote_player;
pub mod renet_config;
pub mod server;

type PlayerIdentifier = usize;

#[derive(Resource)]
pub struct IpResource {
    value: String,
}

#[derive(Resource)]
pub struct PortResource {
    value: u16,
}

#[derive(Component)]
pub struct NetworkIdentifier {
    value: u16,
}
