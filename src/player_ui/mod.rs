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
mod widgets {
    use bevy::prelude::*;
    use stats_and_abilities_system::prelude::Health;

    pub struct HealthBar<T, U>
    where
        T: Component + Clone,
        U: Component + Clone,
    {
        width: f32,
        height: f32,
        pos_left: f32,
        pos_bottom: f32,
        root_marker: T,
        current_health_marker: U,
        health_color: Color,
        background_color: Color,
        font: Option<Handle<Font>>,
    }

    impl<T, U> HealthBar<T, U>
    where
        T: Component + Clone,
        U: Component + Clone,
    {
        pub fn with_font(self, font: Option<Handle<Font>>) -> Self {
            Self { font, ..self }
        }
        pub fn with_width(self, width: f32) -> Self {
            Self { width, ..self }
        }
        pub fn with_height(self, height: f32) -> Self {
            Self { height, ..self }
        }
        pub fn with_pos_left(self, pos_left: f32) -> Self {
            Self { pos_left, ..self }
        }
        pub fn with_pos_bottom(self, pos_bottom: f32) -> Self {
            Self { pos_bottom, ..self }
        }
        pub fn with_health_color(self, health_color: Color) -> Self {
            Self {
                health_color,
                ..self
            }
        }
        pub fn with_background_color(self, background_color: Color) -> Self {
            Self {
                background_color,
                ..self
            }
        }

        pub fn new(root_marker: T, current_health_marker: U) -> Self {
            Self {
                root_marker,
                current_health_marker,
                width: 100.0,
                height: 20.0,
                pos_left: 20.0,
                pos_bottom: 20.0,
                health_color: Color::rgb(1.0, 0.3, 0.3),
                background_color: Color::rgb(0.3, 0.3, 0.3),
                font: None,
            }
        }

        pub fn draw(&self, commands: &mut Commands, health: &Health) {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(self.width), Val::Px(self.height)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(self.pos_left),
                            bottom: Val::Px(self.pos_bottom),
                            ..default()
                        },
                        ..default()
                    },
                    background_color: self.background_color.into(),
                    ..default()
                })
                .insert(self.root_marker.clone())
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Px(self.width * health.get_health_percentage()),
                                    Val::Px(self.height),
                                ),
                                position_type: PositionType::Relative,
                                position: UiRect {
                                    left: Val::Px(0.0),
                                    bottom: Val::Px(0.0),
                                    ..default()
                                },
                                ..default()
                            },
                            background_color: self.health_color.into(),
                            ..default()
                        })
                        .insert(self.current_health_marker.clone());
                })
                .with_children(|parent| {
                    if let Some(ref font_handle) = self.font {
                        parent.spawn(TextBundle::from_section(
                            format!("{}/{}", health.get_current(), health.get_maximum()),
                            TextStyle {
                                font: font_handle.clone_weak(),
                                font_size: 25.0,
                                color: Color::WHITE,
                            },
                        ));
                    };
                });
        }
    }
}
