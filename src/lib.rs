#![allow(dead_code)]

use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::config_management::ConfigPlugin;
use crate::player::PlayerPlugin;
use crate::schedule::CustomSchedulePlugin;
use crate::settings::SettingsPlugin;

pub mod debug;
pub mod settings;
mod player;
mod schedule;
mod config_management;

pub struct Game;


impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(ConfigPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(SettingsPlugin)
            .add_plugins(CustomSchedulePlugin);
    }
}
