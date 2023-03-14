use bevy::prelude::*;
use rust_game::network::server::*;
use rust_game::GameServer;

fn main() {
    App::new().add_plugin(GameServer).run();
}
