use crate::abilities::{aimed_shot, poison_arrow_tick};
use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use crate::settings::controls::ActionBarAction;
use crate::status_event::health_event::{HealthEventQueue, ImmediateStatEvent};
use crate::status_event::ticking_stat_event::{
    TickDuration, TickingStatEvent, TickingStatEventQueue,
};
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
    mut health_event_queues: Query<&mut HealthEventQueue>,
    mut ticking_stat_event: Query<&mut TickingStatEventQueue>,
    player_queue: Query<(Entity, &Target), With<PlayerMarker>>,
) {
    for action in actions.iter() {
        let (player_entity, player_target) = player_queue.get_single().expect("Cannot find player");
        if action.just_pressed(ActionBarAction::Button1) {
            if let Some(target_entity) = player_target.targeted_entity {
                let mut target_event_queue = health_event_queues
                    .get_mut(target_entity)
                    .expect("Target does not exist");

                target_event_queue.events.push(ImmediateStatEvent {
                    target_association: TargetAssociation::new(player_entity, target_entity),
                    apply: aimed_shot,
                });
            }
        } else if action.just_pressed(ActionBarAction::Button2) {
            if let Some(target_entity) = player_target.targeted_entity {
                let mut target_event_queue = ticking_stat_event
                    .get_mut(target_entity)
                    .expect("Target does not exist");

                target_event_queue.push(TickingStatEvent::new(
                    TickDuration::Finite(60),
                    6,
                    ImmediateStatEvent {
                        target_association: TargetAssociation::new(player_entity, target_entity),
                        apply: poison_arrow_tick,
                    },
                ));
            }
        }
    }
}
