mod actor;
mod debug_ui;
mod player;
mod player_ui;

use bevy::prelude::*;

use crate::debug_ui::DebugUI;
use crate::player::*;
use crate::player_ui::PlayerUi;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugUI)
        .add_plugin(PlayerUi)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
