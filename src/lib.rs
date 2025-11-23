use bevy::ecs::system::RunSystemOnce;
use bevy::log::{Level, LogPlugin};
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

pub fn load_bevy_logger(config: &LoadedConf, app: &mut App) {
    let log_plugin = if config.configuration.debug.is_some() {
        if config.configuration.debug.as_ref().unwrap().enable_debug_logging {
            LogPlugin{level: Level::DEBUG, ..default()};
        }
    } else {
        LogPlugin::default();
    };
    app.add_plugins(log_plugin);
    app
        .add_plugins(AssetPlugin::default())
        .add_plugins(WindowPlugin::default());
}

pub fn load_bevy_plugins(config: &LoadedConf, app: &mut App) {
    load_bevy_logger(config, app);
}

pub fn run() {
    let mut app = App::new();

    let mut config_load_error = None;
    let config = match load_config("config"){
        Ok(config) => config,
        Err(err) => {
            config_load_error = Some(err);
            LoadedConf::default()
        }
    };



    app
        .add_plugins(MinimalPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(CustomSchedulePlugin)
        .add_plugins(ConfigPlugin::new(config))
        .add_plugins(DebugPlugin);

    if let Some(ref _err) = config_load_error {
        app.world_mut()
            .run_system_once(|| {
                warn!("Failed to load config");
            })
            .expect("Error running this system.");
    }

    app.run();
}
