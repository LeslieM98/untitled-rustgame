use bevy::prelude::*;
use crate::player::{PlayerInitSet, PlayerSpawnSet};

pub struct CustomSchedulePlugin;

impl Plugin for CustomSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PostStartup, PlayerInitSet.after(PlayerSpawnSet));
    }
}