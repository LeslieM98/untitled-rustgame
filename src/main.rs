mod actor;
mod debug;
mod player_ui;
mod settings;

use crate::actor::enemy::EnemyPlugin;
use bevy::prelude::*;
use debug::DebugPlugin;

use crate::actor::player::*;
use crate::player_ui::PlayerUi;
use crate::settings::SettingsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerUi)
        .add_plugin(EnemyPlugin)
        .add_plugin(SettingsPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let my_gltf = asset_server.load("glTF/Debug_Scene.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..Default::default()
    });
}
