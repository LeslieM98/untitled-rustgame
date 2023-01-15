mod target_tracker;

use crate::player_ui::target_tracker::TargetTrackerUIPlugin;
use bevy::prelude::*;

pub struct PlayerUi;

impl Plugin for PlayerUi {
    fn build(&self, app: &mut App) {
        app.add_plugin(TargetTrackerUIPlugin);
    }
}
