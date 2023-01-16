pub mod controls;

use crate::settings::controls::SettingsControlsPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SettingsControlsPlugin);
    }
}
