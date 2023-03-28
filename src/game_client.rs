use crate::actor::player::PlayerPlugin;
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
            .add_plugin(ClientPlugin::new("127.0.0.1", 42069));
    }
}
