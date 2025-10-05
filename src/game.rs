use crate::player::PlayerPlugin;
use crate::settings::SettingsPlugin;
use bevy::prelude::*;
use crate::load_debug_scene;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(PlayerPlugin)
            .add_plugins(SettingsPlugin)
            .add_systems(Startup, load_debug_scene);
    }
}