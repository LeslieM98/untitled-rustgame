#![allow(dead_code)]

use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::config_management::ConfigPlugin;
use crate::player::PlayerPlugin;
use crate::schedule::CustomSchedulePlugin;
use crate::settings::SettingsPlugin;

pub mod debug;
pub mod settings;
mod player;
mod schedule;
mod config_management;

pub struct Game;


impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(ConfigPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(SettingsPlugin)
            .add_plugins(CustomSchedulePlugin)
            .add_systems(Startup, load_debug_scene);
    }
}

pub fn load_debug_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let my_gltf = asset_server.load("glTF/Debug_Scene.gltf#Scene0");
    let mut light_transform = Transform::from_xyz(0.0, 2.0, 0.0);
    light_transform.rotate_x(-PI / 4.0);

    commands.spawn((DirectionalLight{shadows_enabled: true,..Default::default()}, light_transform));
    commands.spawn((SceneRoot(my_gltf), Transform::from_xyz(2.0, 0.0, -5.0)));
}
