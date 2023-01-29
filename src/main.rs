mod abilities;
mod actor;
mod debug;
mod player_ui;
mod settings;
mod status_event;

use crate::actor::npc::EnemyPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use debug::DebugPlugin;

use crate::actor::player::*;
use crate::player_ui::PlayerUi;
use crate::settings::SettingsPlugin;
use crate::status_event::StatusEventPlugin;

use std::env;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Untiteled Game".to_string(),
            present_mode: PresentMode::Immediate,
            ..default()
        },
        ..default()
    }))
    .add_plugin(PlayerPlugin)
    .add_plugin(PlayerUi)
    .add_plugin(EnemyPlugin)
    .add_plugin(SettingsPlugin)
    .add_plugin(StatusEventPlugin)
    .add_startup_system(setup_scene);

    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug-editor")) {
        app.add_plugin(DebugPlugin);
    }
    app.run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let my_gltf = asset_server.load("glTF/Debug_Scene.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..Default::default()
    });
}
