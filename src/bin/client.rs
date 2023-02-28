use bevy::prelude::App;
use rust_game::GameClient;

fn main() {
    App::new().add_plugin(GameClient).run();
}
