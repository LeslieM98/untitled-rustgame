use bevy::prelude::App;
use rust_game::game_client::*;

fn main() {
    App::new().add_plugin(GameClient).run();
}
