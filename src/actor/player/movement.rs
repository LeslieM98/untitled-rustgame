use crate::actor::player::PlayerMarker;
use bevy::prelude::*;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .label("PlayerMovementSystems")
        .with_system(move_player)
}

fn move_player(
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const VELOCITY: f32 = 3.0;
    let mut direction = Vec3::ZERO;
    let mut transform = query.get_single_mut().unwrap();
    if input.pressed(KeyCode::W) {
        direction += transform.forward();
    }
    if input.pressed(KeyCode::S) {
        direction += transform.back();
    }
    if input.pressed(KeyCode::A) {
        direction += transform.left();
    }
    if input.pressed(KeyCode::D) {
        direction += transform.right();
    }
    if input.pressed(KeyCode::Space) {
        direction += transform.up();
    }
    if input.pressed(KeyCode::LShift) {
        direction += transform.down();
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * VELOCITY * time.delta_seconds();
    }
}
