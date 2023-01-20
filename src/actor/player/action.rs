use crate::actor::target::PlayerTarget;
use crate::settings::controls::ActionBarAction;
use crate::status_event::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn get_system_set() -> SystemSet {
    SystemSet::new().label("PlayerActions").with_system(action)
}

fn action(
    actions: Query<&ActionState<ActionBarAction>>,
    mut target_query: Query<&mut ActionReceivedEventQueue, With<PlayerTarget>>,
) {
    for action in actions.iter() {
        if action.just_pressed(ActionBarAction::Button1) {
            for mut target_queue in &mut target_query {
                target_queue.events.push(ActionReceivedEvent {
                    apply: Box::new(|target_stats| {
                        target_stats.set_current_hp(target_stats.get_current_hp() - 10)
                    }),
                });
            }
        }
    }
}
