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

fn instantiate(commands: &mut Commands, health_percentage: f32) {
    let width = 100.0;
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(100.),
                    bottom: Val::Px(100.),
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
                        size: Size::new(Val::Px(width * health_percentage), Val::Px(20.0)),
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
                .insert(TargetTrackerUIHealthBarMarker);
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
            instantiate(&mut commands, health_percentage);
        }
    }
}
