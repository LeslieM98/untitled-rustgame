use crate::actor::player::PlayerPlugin;
use crate::settings::SettingsPlugin;
use bevy::prelude::*;
use crate::load_debug_scene;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(PlayerPlugin)
            .add_plugins(SettingsPlugin)
            .add_systems(Startup, crate::actor::player::spawn_player)
            .add_systems(Startup, load_debug_scene);
    }
}

// pub fn spawn_player(commands: Commands,
//                     meshes: &mut ResMut<Assets<Mesh>>,
//                     materials: &mut ResMut<Assets<StandardMaterial>>,
//                     asset_server: &Res<AssetServer>) {
//     crate::actor::player::spawn_player(commands, meshes, materials, asset_server);
// }
