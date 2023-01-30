pub mod action;
pub mod camera;
pub mod movement;
pub mod targeting;

use crate::actor::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::{InteractablePickingPlugin, PickingCameraBundle, PickingPlugin};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system_set(movement::get_system_set())
            .add_system_set(camera::get_system_set())
            .add_system_set(targeting::get_system_set())
            .add_system_set(action::get_system_set())
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin);
    }
}

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub actor: Actor,
    pub player: PlayerMarker,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            actor: Default::default(),
            player: PlayerMarker,
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tex_handle = asset_server.load("PNG/Green/texture_04.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tex_handle),
        unlit: false,
        ..default()
    });

    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 1.0, -10.0),
        ..default()
    };
    let mut player_stats = Stats::default();
    player_stats.set_stat(Stats::MOVEMENT_SPEED_MODIFIER, 1000);
    let player_bundle = PlayerBundle {
        actor: Actor {
            pbr,
            stats: player_stats,
            ..default()
        },
        ..default()
    };

    let camera_entity = camera::spawn(&mut commands);
    commands.spawn(player_bundle).add_child(camera_entity);
}
