mod actor;
mod debug;
mod player_ui;

use crate::actor::enemy::EnemyPlugin;
use bevy::prelude::*;
use debug::DebugPlugin;

use crate::actor::player::*;
use crate::player_ui::PlayerUi;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerUi)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    let tex_handle = asset_server.load("PNG/Dark/texture_01.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tex_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    });
    // Floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::default())),
        transform: Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(10.0, 1.0, 10.0),
            ..default()
        },
        material: material_handle,
        ..default()
    });
}
