use crate::actor::player::PlayerPlugin;
use crate::load_debug_scene;
use crate::network::client::ClientPlugin;
use crate::player_ui::PlayerUi;
use crate::settings::SettingsPlugin;
use bevy::prelude::*;

pub struct GameClient;

impl Plugin for GameClient {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(PlayerPlugin)
            .add_plugin(PlayerUi)
            .add_plugin(SettingsPlugin)
            .add_plugin(ClientPlugin::new("127.0.0.1", 42069))
            .add_startup_system(spawn_player)
            .add_startup_system(load_debug_scene);
    }
}

pub fn spawn_player(commands: Commands, asset_server: Res<AssetServer>) {
    let player_model = asset_server.load("glTF/base model/base_model.gltf#Scene0");
    crate::actor::player::spawn_player(commands, player_model);
}
