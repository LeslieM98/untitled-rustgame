use crate::actor::player::camera::PlayerCameraMarker;
use bevy::{diagnostic::Diagnostics, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use super::DebugStage;

pub struct DebugUI;

#[derive(Component)]
pub struct DebugMarker;

impl Plugin for DebugUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin)
            .add_startup_system_to_stage(DebugStage, camera_debug_render);
    }
}

fn camera_debug_render(
    mut command: Commands,
    cams: Query<Entity, With<PlayerCameraMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tex_handle = asset_server.load("PNG/Red/texture_10.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tex_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    });
    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            ..default()
        })),
        material: material_handle,
        ..default()
    };
    for cam_entity in cams.iter() {
        let debug_entity = command.spawn((pbr.clone(), DebugMarker)).id();
        command
            .entity(cam_entity)
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .add_child(debug_entity);
    }
}
