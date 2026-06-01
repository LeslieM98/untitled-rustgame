use bevy::ecs::system::RunSystemOnce;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use crate::config::{ConfigPlugin, LoadedConf};
use crate::debug::DebugPlugin;
use crate::player::PlayerPlugin;
use crate::schedule::CustomSchedulePlugin;
use crate::settings::SettingsPlugin;
use crate::map::MapPlugin;

pub use crate::config::load_config;

pub mod debug;
pub mod settings;
mod player;
mod schedule;
mod config;
mod map;


pub fn load_custom_plugins(config: LoadedConf, app: &mut App) {
    app
        .add_plugins(PlayerPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(CustomSchedulePlugin)
        .add_plugins(ConfigPlugin::new(config))
        .add_plugins(MapPlugin)
        .add_plugins(DebugPlugin);
}
pub fn load_bevy_plugins(config: &LoadedConf, app: &mut App) {
    let mut default_plugins_builder = DefaultPlugins.build();
    if config.configuration.debug.is_some() && config.configuration.debug.as_ref().unwrap().enable_debug_logging {
        default_plugins_builder = default_plugins_builder.set(LogPlugin{level: Level::DEBUG, ..default()});
    }
    app.add_plugins(default_plugins_builder);
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

    load_bevy_plugins(&config, &mut app);
    load_custom_plugins(config, &mut app);


    if let Some(ref _err) = config_load_error {
        app.world_mut()
            .run_system_once(|| {
                warn!("Failed to load config, default config loaded!");
            })
            .expect("Error running this system.");
    }

    for p in app.get_added_plugins::<LogPlugin>(){
        println!("{}", p.level);
    }

    app.run();
}
