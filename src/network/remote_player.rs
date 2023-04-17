use bevy::prelude::{info, Commands, Component, Entity, Query, ResMut, Transform, With};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy_renet::renet::RenetClient;

use crate::actor::{player::PlayerMarker, Actor};
use crate::network::server::MAX_CONNECTIONS;

use super::renet_config::RenetChannel;

#[derive(Component, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
struct MultiplePlayerUpdate {
    content: [Option<(PlayerID, SinglePlayerUpdate)>; MAX_CONNECTIONS],
}

impl MultiplePlayerUpdate {
    pub fn new(
        content: [Option<(PlayerID, SinglePlayerUpdate)>; MAX_CONNECTIONS],
    ) -> MultiplePlayerUpdate {
        Self { content }
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

    fn generate_random_single_player_update() -> SinglePlayerUpdate {
        SinglePlayerUpdate::new(Transform {
            translation: Vec3::new(rand::random(), rand::random(), rand::random()),
            rotation: Quat::from_array([
                rand::random(),
                rand::random(),
                rand::random(),
                rand::random(),
            ]),
            scale: Vec3::new(rand::random(), rand::random(), rand::random()),
        })
    }

    fn generate_random_multiple_player_update_data(
    ) -> [Option<(PlayerID, SinglePlayerUpdate)>; MAX_CONNECTIONS] {
        let mut data = [None; MAX_CONNECTIONS];
        for i in 0..MAX_CONNECTIONS {
            data[i] = Some((
                PlayerID::new(i.try_into().unwrap()),
                generate_random_single_player_update(),
            ));
        }
        data
    }

    #[test]
    fn correct_player_to_server_sync_encoding() {
        let initial = generate_random_single_player_update();

        let data = bincode::serialize(&initial).unwrap();

        let subject = bincode::deserialize(&data).unwrap();

        assert_eq!(initial, subject);
    }

    fn correct_server_to_player_sync_encoding() {
        let mut data = generate_random_multiple_player_update_data();

        let subject = MultiplePlayerUpdate::new(data.clone());
        let serialized = bincode::serialize(&subject).unwrap();
        let deserialized = bincode::deserialize(&serialized).unwrap();
        assert_eq!(subject, deserialized);

        data[0] = None;
        data[data.len() - 1] = None;

        let subject = MultiplePlayerUpdate::new(data.clone());
        let serialized = bincode::serialize(&subject).unwrap();
        let deserialized = bincode::deserialize(&serialized).unwrap();
        assert_eq!(subject, deserialized);

        for x in &mut data {
            *x = None;
        }

        let subject = MultiplePlayerUpdate::new(data.clone());
        let serialized = bincode::serialize(&subject).unwrap();
        let deserialized = bincode::deserialize(&serialized).unwrap();
        assert_eq!(subject, deserialized);
    }
}
