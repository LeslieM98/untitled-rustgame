use bevy::prelude::*;
use rust_game::gamer_server::GameServer;

fn main() {
    App::new().add_plugin(GameServer).run();
}
