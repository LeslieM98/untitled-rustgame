pub mod actor;
pub mod debug;
pub mod player_ui;
pub mod settings;

use crate::actor::npc::EnemyPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use debug::DebugPlugin;

use crate::actor::player::*;
use crate::player_ui::PlayerUi;
use crate::settings::SettingsPlugin;

use std::env;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Untitled Game".to_string(),
                present_mode: PresentMode::Immediate,
                ..default()
            },
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(PlayerUi)
        .add_plugin(EnemyPlugin)
        .add_plugin(SettingsPlugin);
    }
}
