use crate::actor::player::PlayerMarker;
use crate::actor::target::{Target, Targetable};
use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .label("PlayerMovementSystems")
        .with_system(deselect_target)
        .with_system(chose_target)
}

fn deselect_target(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Target, With<PlayerMarker>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut player_target = player_query.get_single_mut().expect("Cannot find player");
        player_target.targeted_entity = None;
    }
}

fn chose_target(
    targetable_query: Query<Entity, With<Targetable>>,
    mut player_query: Query<&mut Target, With<PlayerMarker>>,
    mut events: EventReader<PickingEvent>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            let mut player_target = player_query.get_single_mut().expect("Cannot find player");
            player_target.targeted_entity = if let Ok(target) = targetable_query.get(*e) {
                Some(target)
            } else {
                None
            }
        }
    }
}
