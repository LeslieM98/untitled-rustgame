mod nameplates;

use crate::player_ui::nameplates::NamePlateUIPlugin;
use bevy::prelude::*;

pub struct PlayerUi;

impl Plugin for PlayerUi {
    fn build(&self, app: &mut App) {
        app.add_plugin(NamePlateUIPlugin);
    }
}
