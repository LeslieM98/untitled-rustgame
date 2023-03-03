use bevy::prelude::*;
use rust_game::network::server::*;
use rust_game::GameServer;

fn main() {
    let mut active_connections = Connections::default();
    let connection_server = ConnectionServer::new("localhost", 42069);
    handle_incoming_connections(&mut active_connections, &connection_server);
}
