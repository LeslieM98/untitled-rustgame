use bevy::app::App;
use bevy::log::warn;
use bevy::prelude::{
    info, Commands, Component, CoreSet, Entity, EventWriter, IntoSystemConfig, Plugin, Query, Res,
    Transform, With,
};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};

use crate::actor::{player::PlayerMarker, Actor};
use crate::network::lobby::Lobby;
use crate::network::server::MAX_CONNECTIONS;

use super::client::ClientID;
use super::packet_communication::{
    client_send_packet, server_broadcast_packet, BroadcastPacket, PacketMetaData, PacketType,
    ReceivedMessages, Sender,
};
use crate::network::packet_communication::Sender::Client;

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
        app.add_event::<MultiplePlayerUpdate>()
            .add_system(server_broadcast_packet::<MultiplePlayerUpdate>.in_base_set(CoreSet::Last))
            .add_system(receive_client_to_server_sync)
            .add_system(send_server_to_client_sync);
    }
}

#[derive(Component, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerID {
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

impl BroadcastPacket for MultiplePlayerUpdate {}

impl PacketMetaData for MultiplePlayerUpdate {
    fn get_packet_type() -> PacketType {
        PacketType::ServerToClientPlayerSync
    }

    fn get_content_size(&self) -> u128 {
        bincode::serialized_size(self).unwrap() as u128
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
    recv_messages: Res<ReceivedMessages>,
    mut player_query: Query<&mut Transform>,
    lobby: Res<Lobby>,
) {
    let messages = recv_messages.deserialize::<SinglePlayerUpdate>();

    for (sender, message) in messages {
        let sender_id = match sender {
            Client(id) => id,
            _ => {
                warn! {"Server received a package sent by a Server"};
                continue;
            }
        };
        let entity = match lobby.get_map().get(&sender_id) {
            Some(entity) => entity,
            _ => {
                warn!("Client {} cannot be mapped to an entity", sender_id);
                continue;
            }
        };
        let mut player_transform = match player_query.get_mut(*entity) {
            Ok(transform) => transform,
            Err(err) => {
                warn!(
                    "Could not find the matching entity for id {}: {}",
                    sender_id, err
                );
                continue;
            }
        };
        *player_transform = message.transform;
    }
}

fn send_server_to_client_sync(
    mut event_writer: EventWriter<MultiplePlayerUpdate>,
    player_query: Query<(&PlayerID, &Transform)>,
) {
    let mut player_update = MultiplePlayerUpdate::default();
    for (i, (player_id, transform)) in player_query.iter().enumerate() {
        player_update.content[i] = Some((*player_id, SinglePlayerUpdate::new(*transform)));
    }
    event_writer.send(player_update);
}

fn receive_server_to_client_sync(
    recv_messages: Res<ReceivedMessages>,
    lobby: Res<Lobby>,
    mut player_query: Query<&mut Transform>,
    client_id: Res<ClientID>,
) {
    let messages = recv_messages.deserialize::<MultiplePlayerUpdate>();

    for (sender, message) in messages {
        if sender != Sender::Server {
            warn! {"Client received a package sent by a Client"};
            continue;
        }

        for updates in message.content {
            let (player_id, update) = match updates {
                Some(content) => content,
                _ => continue,
            };
            if player_id.id == client_id.id {
                continue;
            }
            let entity = match lobby.get_map().get(&player_id.id) {
                Some(entity) => entity,
                _ => {
                    warn!("Client {} cannot be mapped to an entity", player_id.id);
                    continue;
                }
            };
            let mut player_transform = match player_query.get_mut(*entity) {
                Ok(transform) => transform,
                Err(err) => {
                    warn!(
                        "Could not find the matching entity for id {}: {}",
                        player_id.id, err
                    );
                    continue;
                }
            };

            *player_transform = update.transform;
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
