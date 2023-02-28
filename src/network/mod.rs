use bevy::prelude::*;

pub mod server;

#[derive(Resource)]
struct IpResource {
    value: String,
}

#[derive(Resource)]
struct PortResource {
    value: u16,
}

#[derive(Component)]
struct NetworkIdentifier {
    value: u16,
}
