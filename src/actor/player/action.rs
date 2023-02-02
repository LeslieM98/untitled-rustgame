use crate::abilities::{aimed_shot, poison_arrow_tick};
use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use crate::settings::controls::ActionBarAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .label("PlayerActions")
        .with_system(player_action)
}

fn player_action(
    actions: Query<&ActionState<ActionBarAction>>,
    player_queue: Query<(Entity, &Target), With<PlayerMarker>>,
) {
    for action in actions.iter() {
        let (player_entity, player_target) = player_queue.get_single().expect("Cannot find player");
        if action.just_pressed(ActionBarAction::Button1) {
            if let Some(target_entity) = player_target.targeted_entity {}
        } else if action.just_pressed(ActionBarAction::Button2) {
            if let Some(target_entity) = player_target.targeted_entity {}
        }
    }
}
