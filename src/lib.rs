#![allow(dead_code)]

pub mod actor;
pub mod debug;
pub mod network;
pub mod player_ui;
pub mod settings;

use crate::actor::npc::EnemyPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

use crate::actor::player::*;
use crate::player_ui::PlayerUi;

use crate::network::server::ServerPlugin;
use crate::settings::SettingsPlugin;

pub struct GameServer;
pub struct GameClient;

impl Plugin for GameServer {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins)
            .add_plugin(LogPlugin::default())
            .add_plugin(ServerPlugin::new("localhost", 42069))
            .add_plugin(EnemyPlugin);
    }
}

impl Plugin for GameClient {
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
        .add_plugin(SettingsPlugin);
    }
}
