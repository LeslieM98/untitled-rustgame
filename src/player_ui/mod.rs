mod nameplates;
mod target_tracker;

use crate::player_ui::nameplates::NamePlateUIPlugin;
use crate::player_ui::target_tracker::TargetTrackerUIPlugin;
use bevy::prelude::*;

pub struct PlayerUi;

impl Plugin for PlayerUi {
    fn build(&self, app: &mut App) {
        app.add_plugin(NamePlateUIPlugin)
            .add_plugin(TargetTrackerUIPlugin);
    }
}

pub fn instantiate_health_bar<T, U>(
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
