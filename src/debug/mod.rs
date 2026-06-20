use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;
use bevy::settings::*;

pub struct DebugPlugin;
#[derive(Resource, SettingsGroup, Reflect, Default)]
#[reflect(Resource, SettingsGroup, Default)]
struct DebugSettings {
    enable_debug_log: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, debug_info);
    }
}

pub fn debug_info() {
    debug!("Debug Plugin loaded!");

}
