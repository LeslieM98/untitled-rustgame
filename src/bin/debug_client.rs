use bevy::app::App;
use rust_game::game_client::GameClient;

fn main() {
    App::new().add_plugin(GameClient).run();
}
