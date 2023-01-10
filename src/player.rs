use crate::actor::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Bundle)]
pub struct Player {
    pub actor: Actor,
    pub player: PlayerMarker,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            actor: Default::default(),
            player: PlayerMarker,
        }
    }
}

pub fn move_player(
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const VELOCITY: f32 = 1.0;
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::S) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::A) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::D) {
        direction.z -= 1.0;
    }

    if direction.length() > 0.001 {
        direction = direction.normalize();
        let mut transform = query.get_single_mut().unwrap();
        transform.translation += direction * VELOCITY * time.delta_seconds();
    }
}

pub fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
        ..default()
    };

    commands
        .spawn(Player {
            actor: Actor { pbr, ..default() },
            ..default()
        })
        .id()
        .index();
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}
