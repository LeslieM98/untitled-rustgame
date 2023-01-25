use crate::actor::player::camera::PlayerCameraMarker;
use crate::actor::player::PlayerMarker;
use crate::actor::target::{Target, Targetable};
use crate::status_event::stats::*;
use bevy::app::App;
use bevy::prelude::*;

pub struct TargetTrackerUIPlugin;

#[derive(Component)]
pub struct TargetTrackerUIMarker;
#[derive(Component)]
pub struct TargetTrackerUIHealthMarker;

impl Plugin for TargetTrackerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_target_selected)
            .add_system(refresh)
            .add_system_to_stage(CoreStage::PostUpdate, on_target_deselected);
    }
}

fn instantiate(commands: &mut Commands, stats: &Stats, position: &Vec2) {
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
                        size: Size::new(Val::Px(width * stats.get_hp_percentage()), Val::Px(20.0)),
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

fn refresh(
    mut commands: Commands,
    ui_query: Query<Entity, With<TargetTrackerUIMarker>>,
    stat_query: Query<(
        &GlobalTransform,
        ChangeTrackers<GlobalTransform>,
        &Stats,
        ChangeTrackers<Stats>,
    )>,
    target_query: Query<&Target, With<PlayerMarker>>,
    camera_query: Query<
        (&Camera, &GlobalTransform, ChangeTrackers<GlobalTransform>),
        With<PlayerCameraMarker>,
    >,
) {
    for ui in ui_query.iter() {
        if let Some(target) = target_query
            .get_single()
            .expect("Cannot find player")
            .targeted_entity
        {
            for (camera, camera_transform, camera_transform_tracker) in camera_query.iter() {
                let (transform, transform_tracker, stat_instance, stat_tracker) =
                    stat_query.get(target).expect("Cannot find target");
                if transform_tracker.is_changed()
                    || stat_tracker.is_changed()
                    || camera_transform_tracker.is_changed()
                {
                    let ui_pos =
                        camera.world_to_viewport(camera_transform, transform.translation());
                    match ui_pos {
                        Some(pos) => {
                            commands.entity(ui).despawn_recursive();
                            instantiate(&mut commands, stat_instance, &pos);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
fn on_target_selected(
    mut commands: Commands,
    targetable_query: Query<(&GlobalTransform, &Stats), With<Targetable>>,
    player_query: Query<&Target, (Changed<Target>, With<PlayerMarker>)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCameraMarker>>,
) {
    if !player_query.is_empty() {
        let player_target = player_query.get_single().expect("Cannot find player");
        if let Some(target) = player_target.targeted_entity {
            let (target_transform, target_stats) = targetable_query.get(target).unwrap();
            let (camera, camera_transform) = camera_query.get_single().unwrap();
            let ui_pos = camera.world_to_viewport(camera_transform, target_transform.translation());
            match ui_pos {
                Some(pos) => instantiate(&mut commands, target_stats, &pos),
                _ => {}
            }
        }
    }
}

fn on_target_deselected(
    mut commands: Commands,
    old_ui: Query<Entity, With<TargetTrackerUIMarker>>,
    changed_targets: Query<&Target, Changed<Target>>,
) {
    for _ in changed_targets.iter() {
        for ui in old_ui.iter() {
            commands.entity(ui).despawn_recursive();
        }
    }
}
