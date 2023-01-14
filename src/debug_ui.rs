use crate::player::{CameraBaseMarker, PlayerMarker};
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct DebugUI;

#[derive(Component)]
pub struct DebugMarker;

#[derive(StageLabel)]
pub struct DebugStage;

impl Plugin for DebugUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin)
            .add_startup_stage_after(
                StartupStage::PostStartup,
                DebugStage,
                SystemStage::single_threaded(),
            )
            .add_startup_system_to_stage(DebugStage, camera_debug_render)
            .add_startup_system_to_stage(DebugStage, empty_transform_debug_render);
    }
}

fn camera_debug_render(
    mut command: Commands,
    cams: Query<Entity, With<Camera3d>>,
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

fn empty_transform_debug_render(
    mut command: Commands,
    cams: Query<Entity, With<CameraBaseMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tex_handle = asset_server.load("PNG/Orange/texture_10.png");
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
    for entity in cams.iter() {
        let debug_entity = command.spawn((pbr.clone(), DebugMarker)).id();
        command
            .entity(entity)
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .add_child(debug_entity);
    }
}
