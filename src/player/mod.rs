pub mod camera;
pub mod movement;

use std::io::SeekFrom::Start;
use bevy::app::MainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use crate::player::movement::{move_player, PlayerMovementPlugin};
use bevy::prelude::*;
use bevy::math::*;
use crate::player::camera::CameraPlugin;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PlayerSpawn;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PlayerInit;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(PlayerInit);
        app.init_schedule(PlayerSpawn);

        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
        order.insert_startup_after(PostStartup, PlayerSpawn);
        order.insert_startup_after(PlayerSpawn, PlayerInit);

        app
            .add_systems(PlayerSpawn, spawn_player)
            .add_systems(PlayerInit, init_mesh)
            .add_systems(Update, move_player.in_set(PlayerControlSet));

        app.add_plugins(CameraPlugin)
            .add_plugins(PlayerMovementPlugin);
    }
}

#[derive(SystemSet, Eq, Clone, Copy, PartialEq, Hash, Debug)]
struct PlayerControlSet;

#[derive(Component, Default)]
pub struct PlayerMarker(u32);


pub fn spawn_player(mut commands: Commands){
    commands.spawn((PlayerMarker::default(), Transform::default()));
}

pub fn init_mesh(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    asset_server: Res<AssetServer>,
                    mut materials: ResMut<Assets<StandardMaterial>>,
                    player: Query<Entity, With<PlayerMarker>>){
    let player_entity = player.single().unwrap();
    let tex_handle = asset_server.load("PNG/Purple/texture_04.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tex_handle.clone()),
        unlit: false,
        ..default()
    });

    let default_mesh = Mesh3d(meshes.add(Capsule3d::default()));
    let material = MeshMaterial3d(material_handle);

    commands
        .entity(player_entity)
        .insert((default_mesh, material));
}