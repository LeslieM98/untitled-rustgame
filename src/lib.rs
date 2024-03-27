#![allow(dead_code)]

use bevy::asset::AssetServer;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{default, Commands, Res, SceneBundle, Transform};
use std::f32::consts::PI;

pub mod actor;
pub mod debug;
pub mod settings;
pub mod game;

pub fn load_debug_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let my_gltf = asset_server.load("glTF/Debug_Scene.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..Default::default()
    });

    let sun = DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    };

    commands.spawn(sun);
}
