use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{debug};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, debug_info);
    }
}

pub fn debug_info() {
    debug!("Debug Plugin loaded!");
}
