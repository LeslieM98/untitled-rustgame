use crate::actor::player::PlayerMarker;
use crate::settings::controls::MovementAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .label("PlayerMovementSystems")
        .with_system(move_player)
}

fn move_player(
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    inputs: Query<&ActionState<MovementAction>>,
    time: Res<Time>,
) {
    const VELOCITY: f32 = 3.0;
    let mut direction = Vec3::ZERO;
    let mut transform = query.get_single_mut().unwrap();
    for input in inputs.iter() {
        if input.pressed(MovementAction::Forward) {
            direction += transform.forward();
        }
        if input.pressed(MovementAction::Backward) {
            direction += transform.back();
        }
        if input.pressed(MovementAction::Left) {
            direction += transform.left();
        }
        if input.pressed(MovementAction::Right) {
            direction += transform.right();
        }
        if input.pressed(MovementAction::Jump) {
            direction += transform.up();
        }
        if input.pressed(MovementAction::Crouch) {
            direction += transform.down();
        }
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * VELOCITY * time.delta_seconds();
    }
}
