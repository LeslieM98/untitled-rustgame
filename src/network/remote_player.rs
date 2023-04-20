use bevy::app::App;
use bevy::log::warn;
use bevy::prelude::{
    info, Commands, Component, CoreSet, Entity, EventWriter, IntoSystemConfig, Plugin, Query, Res,
    ResMut, Transform, With,
};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy_renet::renet::{RenetClient, RenetServer};

use crate::actor::{player::PlayerMarker, Actor};
use crate::network::lobby::Lobby;
use crate::network::server::MAX_CONNECTIONS;

use super::packet_communication::{
    client_send_packet, server_recv_packet, PacketMetaData, PacketType,
};
use super::renet_config::RenetChannel;

pub struct ClientPlayerSyncPlugin;

impl Plugin for ClientPlayerSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SinglePlayerUpdate>()
            .add_system(sync_client_to_server)
            .add_system(receive_server_to_client_sync)
            .add_system(client_send_packet::<SinglePlayerUpdate>.in_base_set(CoreSet::Last));
    }
}

pub struct ServerPlayerSyncPlugin;

impl Plugin for ServerPlayerSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(receive_client_to_server_sync)
            .add_system(send_server_to_client_sync);
    }
}

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

impl PacketMetaData for SinglePlayerUpdate {
    fn get_packet_type() -> super::packet_communication::PacketType {
        PacketType::ClientToServerPlayerSync
    }

    fn get_content_size(&self) -> u128 {
        bincode::serialized_size(self).unwrap() as u128
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
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

fn sync_client_to_server(
    player_transform_query: Query<&Transform, With<PlayerMarker>>,
    mut events: EventWriter<SinglePlayerUpdate>,
) {
    let transform = player_transform_query
        .get_single()
        .expect("Player character not found");

    let player_update = SinglePlayerUpdate {
        transform: *transform,
    };

    events.send(player_update);
}

fn receive_client_to_server_sync(
    mut server: ResMut<RenetServer>,
    mut player_query: Query<&mut Transform>,
    lobby: Res<Lobby>,
) {
    for (id, entity) in lobby.get_map() {
        while let Some(packet) = server.receive_message(*id, RenetChannel::PlayerToServerSync) {
            let deserialized: SinglePlayerUpdate = bincode::deserialize(&packet)
                .map_err(|e| warn!("{}", e))
                .unwrap();
            *player_query.get_mut(*entity).unwrap() = deserialized.transform;
        }
    }
}

fn send_server_to_client_sync(
    mut server: ResMut<RenetServer>,
    player_query: Query<(&PlayerID, &Transform)>,
) {
    let mut player_update = MultiplePlayerUpdate::default();
    for (i, (player_id, transform)) in player_query.iter().enumerate() {
        player_update.content[i] = Some((*player_id, SinglePlayerUpdate::new(*transform)));
    }

    let serialized = bincode::serialize(&player_update).unwrap();
    server.broadcast_message(RenetChannel::ServerToClientSync, serialized);
}

fn receive_server_to_client_sync(
    mut client: ResMut<RenetClient>,
    lobby: Res<Lobby>,
    mut player_query: Query<&mut Transform>,
) {
    while let Some(msg) = client.receive_message(RenetChannel::ServerToClientSync) {
        let updated_content: MultiplePlayerUpdate = bincode::deserialize(&msg).unwrap();

        let lobby_map = lobby.get_map();
        for update in updated_content.content {
            let (id, updated_transform) = if update == None {
                return;
            } else {
                update.unwrap()
            };

            if lobby_map.contains_key(&id.id) {
                let mut transform = player_query
                    .get_mut(*lobby_map.get(&id.id).unwrap())
                    .unwrap();
                *transform = updated_transform.transform;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Quat, Vec3};

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