use crate::settings::controls::MovementAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use crate::player::{PlayerControlSet, PlayerMarker};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.in_set(PlayerControlSet));
    }
}

pub fn move_player(
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    inputs: Query<&ActionState<MovementAction>>,
    time: Res<Time>,
) {
    for input in inputs.iter() {
        for mut transform in query.iter_mut() {
            let mut direction = Vec3::ZERO;

            if input.pressed(&MovementAction::Forward) {
                direction += *transform.forward();
            }
            if input.pressed(&MovementAction::Backward) {
                direction += *transform.back();
            }
            if input.pressed(&MovementAction::Left) {
                direction += *transform.left();
            }
            if input.pressed(&MovementAction::Right) {
                direction += *transform.right();
            }
            if input.pressed(&MovementAction::Jump) {
                direction += *transform.up();
            }
            if input.pressed(&MovementAction::Crouch) {
                direction += *transform.down();
            }

            if direction != Vec3::ZERO {
                direction = direction.normalize();
                transform.translation += direction * 6.0 * time.delta_secs();
            }
        }
    }
}
