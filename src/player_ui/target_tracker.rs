use crate::actor::health::BaseHealth;
use crate::actor::player::camera::PlayerCameraMarker;
use crate::actor::player::PlayerMarker;
use crate::actor::target::PlayerTarget;
use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier3d::parry::transformation::utils::transform;

pub struct TargetTrackerUIPlugin;

#[derive(Component)]
pub struct TargetTrackerUIMarker;
#[derive(Component)]
pub struct TargetTrackerUIHealthMarker;

impl Plugin for TargetTrackerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_target_selected)
            .add_system(refresh_on_health_change)
            .add_system(refresh_on_camera_movement)
            .add_system_to_stage(CoreStage::PostUpdate, on_target_deselected);
    }
}

fn instantiate(mut commands: &mut Commands, health: &BaseHealth, position: &Vec2) {
    let width = 100.0;
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(position.x),
                    bottom: Val::Px(position.y),
                    ..default()
                },
                ..default()
            },
            background_color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..default()
        })
        .insert(TargetTrackerUIMarker)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(width * health.get_percentage()), Val::Px(20.0)),
                        position_type: PositionType::Relative,
                        position: UiRect {
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                    background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                    ..default()
                })
                .insert(TargetTrackerUIHealthMarker);
        });
}

fn refresh_on_health_change(
    mut commands: Commands,
    ui_query: Query<Entity, With<TargetTrackerUIHealthMarker>>,
    health_query: Query<(&GlobalTransform, &BaseHealth), (With<PlayerTarget>, Changed<BaseHealth>)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCameraMarker>>,
) {
    if !ui_query.is_empty() {
        let ui = ui_query.get_single().expect("No ui found");
        let (camera, camera_transform) = camera_query.get_single().expect("No camera found");
        for (transform, health_instance) in health_query.iter() {
            let ui_pos = camera.world_to_viewport(camera_transform, transform.translation());
            match ui_pos {
                Some(pos) => {
                    commands.entity(ui).despawn_recursive();
                    instantiate(&mut commands, health_instance, &pos);
                }
                _ => {}
            }
        }
    }
}

fn refresh_on_camera_movement(
    mut commands: Commands,
    ui_query: Query<Entity, With<TargetTrackerUIHealthMarker>>,
    health_query: Query<(&GlobalTransform, &BaseHealth), With<PlayerTarget>>,
    camera_query: Query<
        (&Camera, &GlobalTransform),
        (With<PlayerCameraMarker>, Changed<GlobalTransform>),
    >,
) {
    if !ui_query.is_empty() {
        let ui = ui_query.get_single().unwrap();
        for (camera, camera_transform) in camera_query.iter() {
            for (transform, health_instance) in health_query.iter() {
                let ui_pos = camera.world_to_viewport(camera_transform, transform.translation());
                match ui_pos {
                    Some(pos) => {
                        commands.entity(ui).despawn_recursive();
                        instantiate(&mut commands, health_instance, &pos);
                    }
                    _ => {}
                }
            }
        }
    }
}
fn on_target_selected(
    mut commands: Commands,
    target: Query<(&GlobalTransform, &BaseHealth), Changed<PlayerTarget>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCameraMarker>>,
) {
    if !target.is_empty() {
        let (target_transform, target_health) = target.get_single().unwrap();
        let (camera, camera_transform) = camera_query.get_single().unwrap();
        let ui_pos = camera.world_to_viewport(camera_transform, target_transform.translation());
        match ui_pos {
            Some(pos) => instantiate(&mut commands, target_health, &pos),
            _ => {}
        }
    }
}

fn on_target_deselected(
    mut commands: Commands,
    old_ui: Query<Entity, With<TargetTrackerUIMarker>>,
    removed_targets: RemovedComponents<PlayerTarget>,
) {
    for _ in removed_targets.iter() {
        for ui in old_ui.iter() {
            commands.entity(ui).despawn_recursive();
        }
    }
}
