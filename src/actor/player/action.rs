use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use crate::settings::controls::ActionBarAction;
use crate::status_event::damage_event::{DamageEvent, DamageEventQueue};
use crate::status_event::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .label("PlayerActions")
        .with_system(player_action)
}

fn player_action(
    actions: Query<&ActionState<ActionBarAction>>,
    mut target_query: Query<&mut DamageEventQueue>,
    player_queue: Query<(Entity, &Target), With<PlayerMarker>>,
) {
    for action in actions.iter() {
        if action.just_pressed(ActionBarAction::Button1) {
            let (player_entity, player_target) =
                player_queue.get_single().expect("Cannot find player");
            if let Some(target_entity) = player_target.targeted_entity {
                let mut target_event_queue = target_query
                    .get_mut(target_entity)
                    .expect("Target does not exist");

                target_event_queue.events.push(DamageEvent {
                    target_association: TargetAssociation::new(player_entity, target_entity),
                    apply: Box::new(|_source_stats, target_stats| {
                        target_stats.set_current_hp(target_stats.get_current_hp() - 10)
                    }),
                });
            }
        }
    }
}
