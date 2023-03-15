pub mod action;
pub mod camera;
pub mod movement;
pub mod targeting;

use crate::actor::player::action::player_action;
use crate::actor::player::camera::{camera_scroll, orbit_camera};
use crate::actor::player::movement::move_player;
use crate::actor::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(SystemSet, Eq, Clone, Copy, PartialEq, Hash, Debug)]
struct PlayerControlSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation)
            .add_system(move_player.in_set(PlayerControlSet))
            .add_systems((orbit_camera, camera_scroll).in_set(PlayerControlSet))
            // .add_systems((chose_target, deselect_target).in_set(PlayerControlSet)) TODO!
            .add_system(player_action.in_set(PlayerControlSet));
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

pub fn spawn_player(mut commands: Commands, player_model: Handle<Scene>) {
    let player_stats = StatBlock::default();
    let player_bundle = PlayerBundle {
        actor: Actor {
            stats: player_stats,
            ..default()
        },
        ..default()
    };

    let player_scene = SceneBundle {
        scene: player_model,
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
