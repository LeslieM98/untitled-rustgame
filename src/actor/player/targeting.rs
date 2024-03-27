use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use bevy::prelude::*;

pub fn deselect_target(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Target, With<PlayerMarker>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut player_target = player_query.get_single_mut().expect("Cannot find player");
        player_target.targeted_entity = None;
    }
}

pub fn chose_target() {
    todo!()
}
