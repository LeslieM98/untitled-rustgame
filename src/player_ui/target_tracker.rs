use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use crate::status_event::stats::Stats;
use bevy::prelude::*;

pub struct TargetTrackerUIPlugin;

impl Plugin for TargetTrackerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, draw);
    }
}

#[derive(Component)]
struct TargetTrackerUIMarker;
#[derive(Component)]
struct TargetTrackerUIHealthBarMarker;

fn instantiate<T, U>(
    commands: &mut Commands,
    health_percentage: f32,
    width: f32,
    height: f32,
    pos_left: f32,
    pos_bottom: f32,
    root_marker: T,
    current_health_marker: U,
    health_color: Color,
    background_color: Color,
) where
    T: Component,
    U: Component,
{
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(height)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(pos_left),
                    bottom: Val::Px(pos_bottom),
                    ..default()
                },
                ..default()
            },
            background_color: background_color.into(),
            ..default()
        })
        .insert(root_marker)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(width * health_percentage), Val::Px(height)),
                        position_type: PositionType::Relative,
                        position: UiRect {
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                    background_color: health_color.into(),
                    ..default()
                })
                .insert(current_health_marker);
        });
}

fn draw(
    mut commands: Commands,
    stats_query: Query<&Stats>,
    tracker_query: Query<&Target, With<PlayerMarker>>,
    ui_query: Query<Entity, With<TargetTrackerUIMarker>>,
) {
    ui_query.for_each(|ui_element| commands.entity(ui_element).despawn_recursive());

    for tracker in &tracker_query {
        if let Some(target) = tracker.targeted_entity {
            let health_percentage = stats_query
                .get(target)
                .expect("Cannot find target")
                .get_hp_percentage();
            instantiate(
                &mut commands,
                health_percentage,
                100.0,
                100.0,
                0.0,
                0.0,
                TargetTrackerUIMarker,
                TargetTrackerUIHealthBarMarker,
                Color::rgb(1.0, 0.3, 0.3),
                Color::rgb(0.3, 0.3, 0.3),
            );
        }
    }
}
