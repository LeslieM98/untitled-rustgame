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
            .add_system(animation)
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

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut player_stats = StatBlock::default();
    let player_bundle = PlayerBundle {
        actor: Actor {
            stats: player_stats,
            ..default()
        },
        ..default()
    };

    let player_scene = SceneBundle {
        scene: asset_server.load("glTF/base model/base_model.gltf#Scene0"),
        ..default()
    };

    let camera_entity = camera::spawn(&mut commands);
    commands
        .spawn(player_bundle)
        .insert(player_scene)
        .add_child(camera_entity);
}

pub fn animation(
    asset_server: Res<AssetServer>,
    mut anim_player_query: Query<&mut AnimationPlayer>,
    mut already_started: Local<bool>,
) {
    if !(*already_started) {
        let animation = asset_server.load("glTF/base model/base_model.gltf#Animation1");
        for mut anim_player in &mut anim_player_query {
            info!("Anim started");
            anim_player.play(animation.clone_weak()).repeat();
            *already_started = true;
        }
    }
}
