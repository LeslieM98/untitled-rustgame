use bevy::app::App;
use rust_game::game_client::GameClient;

fn main() {
    App::new()
        .add_plugin(GameClient)
        .add_plugin(bevy_editor_pls::EditorPlugin::default())
        .run();
}
