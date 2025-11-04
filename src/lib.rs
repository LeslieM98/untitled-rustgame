use bevy::prelude::*;
use crate::config::{ConfigPlugin, LoadedConf};
use crate::debug::DebugPlugin;
use crate::player::PlayerPlugin;
use crate::schedule::CustomSchedulePlugin;
use crate::settings::SettingsPlugin;

pub mod debug;
pub mod settings;
mod player;
mod schedule;
mod config;

pub use crate::config::load_config;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(PlayerPlugin)
            .add_plugins(SettingsPlugin)
            .add_plugins(CustomSchedulePlugin);
    }
}

pub fn run(configuration: Result<LoadedConf, std::io::Error>) {
    let mut app = App::new();

    app.add_plugins(Game)
        .add_plugins(ConfigPlugin::new(configuration))
        .add_plugins(DebugPlugin);
    app.run();
}
