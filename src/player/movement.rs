use bevy::prelude::*;
use crate::player::{PlayerControlSet, PlayerMarker};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.in_set(PlayerControlSet));
    }
}

pub fn move_player(
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    // inputs: Query<&ActionState<MovementAction>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if key.pressed(KeyCode::KeyW) {
            direction += *transform.forward();
        }
        if key.pressed(KeyCode::KeyS) {
            direction += *transform.back();
        }
        if key.pressed(KeyCode::KeyA) {
            direction += *transform.left();
        }
        if key.pressed(KeyCode::KeyD) {
            direction += *transform.right();
        }
        if key.pressed(KeyCode::Space) {
            direction += *transform.up();
        }
        if key.pressed(KeyCode::ControlLeft) {
            direction += *transform.down();
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * 6.0 * time.delta_secs();
        }
    }

}
