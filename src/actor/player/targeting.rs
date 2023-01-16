use crate::actor::target::PlayerTarget;
use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .label("PlayerMovementSystems")
        .with_system(deselect_target)
        .with_system(chose_target)
}

fn deselect_target(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    current_target: Query<Entity, With<PlayerTarget>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if !current_target.is_empty() {
            let entity = current_target.get_single().unwrap();
            commands.entity(entity).remove::<PlayerTarget>();
        }
    }
}

fn chose_target(
    mut commands: Commands,
    mut current_target: Query<Entity, With<PlayerTarget>>,
    mut events: EventReader<PickingEvent>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                for selected_target in current_target.iter_mut() {
                    commands.entity(selected_target).remove::<PlayerTarget>();
                }
                commands.entity(*e).insert(PlayerTarget);
            }
            _ => {}
        }
    }
}
