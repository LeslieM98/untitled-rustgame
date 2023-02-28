use bevy::prelude::*;
use std::net::UdpSocket;

pub mod server;

#[derive(Resource)]
struct IpResource {
    value: String,
}

#[derive(Resource)]
struct PortResource {
    value: u32,
}
