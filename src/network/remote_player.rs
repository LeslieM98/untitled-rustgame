use bevy::prelude::{info, Commands, Component, Entity, Query, ResMut, Transform, With};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy_renet::renet::RenetClient;

use crate::actor::{player::PlayerMarker, Actor};

use super::renet_config::RenetChannel;

#[derive(Component)]
struct PlayerID {
    id: u64,
}

impl PlayerID {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
struct SinglePlayerUpdate {
    transform: Transform,
}

impl SinglePlayerUpdate {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}

pub fn spawn_remote_player(commands: &mut Commands, client_id: u64) -> Entity {
    info!("Spawning client {}", client_id);
    let remote_player = Actor::default();
    commands
        .spawn(remote_player)
        .insert(PlayerID::new(client_id))
        .id()
}

fn sync_player_to_server(
    player_transform_query: Query<&Transform, With<PlayerMarker>>,
    mut client: ResMut<RenetClient>,
) {
    let transform = player_transform_query
        .get_single()
        .expect("Player character not found");

    let player_update = SinglePlayerUpdate {
        transform: *transform,
    };

    let payload = bincode::serialize(&player_update).unwrap();

    client.send_message(RenetChannel::PlayerToServerSync, payload);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{EulerRot, Quat, Vec3};

    use super::*;

    #[test]
    fn correct_player_to_server_sync_encoding() {
        let transform = Transform {
            translation: Vec3::new(123.0, 444.0, 420.0),
            scale: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::from_euler(EulerRot::XYZ, 432.0, 756.0, 1423.0),
        };
        let initial = SinglePlayerUpdate::new(transform);

        let data = bincode::serialize(&initial).unwrap();

        let subject = bincode::deserialize(&data).unwrap();

        assert_eq!(initial, subject);
    }
}
