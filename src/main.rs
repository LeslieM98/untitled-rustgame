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

fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::default())),
        transform: Transform {
            translation: Vec3::ZERO,
            ..default()
        },
        ..default()
    });
}
