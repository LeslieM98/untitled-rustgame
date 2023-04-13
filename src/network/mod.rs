use bevy::prelude::*;

pub mod client;
pub mod lobby;
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
