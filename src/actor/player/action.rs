use crate::actor::health::BaseHealth;
use crate::actor::target::PlayerTarget;
use crate::settings::controls::ActionBarAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn get_system_set() -> SystemSet {
    SystemSet::new().label("PlayerActions").with_system(action)
}

fn action(
    actions: Query<&ActionState<ActionBarAction>>,
    mut target_query: Query<&mut BaseHealth, With<PlayerTarget>>,
) {
    for mut health in target_query.iter_mut() {
        for action in actions.iter() {
            if action.just_pressed(ActionBarAction::Button1) {
                health.apply_damage(10);
            }
        }
    }
}
