use bevy::app::App;
use rust_game::network::client::*;
use rust_game::GameClient;

fn main() {
    App::new().add_plugin(GameClient).run();
}
