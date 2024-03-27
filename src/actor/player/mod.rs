pub mod camera;
pub mod movement;
pub mod targeting;

use crate::actor::player::camera::{camera_scroll, orbit_camera};
use crate::actor::player::movement::move_player;
use crate::actor::{Actor, Name};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::math::*;

pub struct PlayerPlugin;

#[derive(SystemSet, Eq, Clone, Copy, PartialEq, Hash, Debug)]
struct PlayerControlSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.in_set(PlayerControlSet))
            .add_systems(Update, (orbit_camera, camera_scroll).in_set(PlayerControlSet));
        // .add_systems((chose_target, deselect_target).in_set(PlayerControlSet)) TODO!
    }
}

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub actor: Actor,
    pub player: PlayerMarker,
}

impl PlayerBundle {
    pub fn with_name(self, name: Name) -> PlayerBundle {
        PlayerBundle {
            actor: Actor { name, ..self.actor },
            ..self
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            actor: Default::default(),
            player: PlayerMarker,
        }
    }
}

pub fn spawn_player(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    asset_server: Res<AssetServer>,
                    mut materials: ResMut<Assets<StandardMaterial>>) {
    let player_bundle = PlayerBundle {
        actor: Actor {
            ..default()
        },
        ..default()
    };


    let tex_handle = asset_server.load("PNG/Purple/texture_04.png");


    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tex_handle.clone()),
        unlit: false,
        ..default()
    });

    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(Capsule3d::default())),
        material: material_handle,
        ..default()
    };

    let camera_entity = camera::spawn(&mut commands);
    commands
        .spawn(player_bundle)
        .insert(pbr)
        .add_child(camera_entity);
}