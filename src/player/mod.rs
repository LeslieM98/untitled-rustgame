pub mod camera;
pub mod movement;

use crate::player::camera::{camera_scroll, orbit_camera};
use crate::player::movement::move_player;
use bevy::prelude::*;
use bevy::math::*;

pub struct PlayerPlugin;

#[derive(SystemSet, Eq, Clone, Copy, PartialEq, Hash, Debug)]
struct PlayerControlSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(PostStartup, init_camera)
            .add_systems(PostStartup, init_mesh)
            .add_systems(Update, move_player.in_set(PlayerControlSet))
            .add_systems(Update, (orbit_camera, camera_scroll).in_set(PlayerControlSet));
    }
}

#[derive(Component)]
pub struct PlayerMarker(u32);

impl Default for PlayerMarker{
    fn default() -> Self {
        PlayerMarker(0)
    }
}

pub fn spawn_player(mut commands: Commands){
    commands.spawn((PlayerMarker(0), Transform::default()));
}

pub fn init_camera(mut commands: Commands,
                   mut player: Query<Entity, With<PlayerMarker>>){
    let player_entity = player.single().unwrap();
    let camera_entity = camera::spawn(&mut commands);
    commands.entity(player_entity).add_child(camera_entity);
}

pub fn init_mesh(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    asset_server: Res<AssetServer>,
                    mut materials: ResMut<Assets<StandardMaterial>>,
                    mut player: Query<Entity, With<PlayerMarker>>){
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