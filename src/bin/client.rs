use bevy::prelude::App;
use rust_game::Game;

fn main() {
    App::new().add_plugin(Game).run();
}
