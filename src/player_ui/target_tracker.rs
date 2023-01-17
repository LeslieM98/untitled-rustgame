use crate::actor::health::BaseHealth;
use crate::actor::player::PlayerMarker;
use crate::actor::target::PlayerTarget;
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

fn instantiate(mut commands: &mut Commands, health: &BaseHealth) {
    let width = 100.0;
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(20.0),
                    bottom: Val::Px(20.0),
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

fn refresh(
    mut commands: Commands,
    ui_query: Query<Entity, With<TargetTrackerUIHealthMarker>>,
    health_query: Query<&BaseHealth, (With<PlayerTarget>, Changed<BaseHealth>)>,
) {
    if !ui_query.is_empty() {
        let ui = ui_query.get_single().unwrap();
        for health_instance in health_query.iter() {
            commands.entity(ui).despawn_recursive();

            instantiate(&mut commands, health_instance);
        }
    }
}
fn on_target_selected(mut commands: Commands, target: Query<&BaseHealth, Changed<PlayerTarget>>) {
    if !target.is_empty() {
        let unwrapped = target.get_single().unwrap();
        instantiate(&mut commands, unwrapped);
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
