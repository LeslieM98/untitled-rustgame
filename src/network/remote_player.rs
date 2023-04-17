use bevy::app::App;
use bevy::log::warn;
use bevy::prelude::{
    info, Commands, Component, Entity, Plugin, Query, Res, ResMut, Transform, With,
};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy_renet::renet::{RenetClient, RenetServer};

use crate::actor::{player::PlayerMarker, Actor};
use crate::network::client::ClientID;
use crate::network::lobby::Lobby;
use crate::network::remote_player::PlayerSyncPacket::ServerToClient;
use crate::network::server::MAX_CONNECTIONS;

use super::renet_config::RenetChannel;

pub struct ClientPlayerSyncPlugin;

impl Plugin for ClientPlayerSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(send_client_to_server_sync)
            .add_system(receive_server_to_client_sync);
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
enum PlayerSyncPacket {
    ServerToClient(MultiplePlayerUpdate),
    ClientToServer(SinglePlayerUpdate),
}

pub fn spawn_remote_player(commands: &mut Commands, client_id: u64) -> Entity {
    info!("Spawning client {}", client_id);
    let remote_player = Actor::default();
    commands
        .spawn(remote_player)
        .insert(PlayerID::new(client_id))
        .id()
}

fn send_client_to_server_sync(
    player_transform_query: Query<&Transform, With<PlayerMarker>>,
    mut client: ResMut<RenetClient>,
) {
    let transform = player_transform_query
        .get_single()
        .expect("Player character not found");

    let player_update = PlayerSyncPacket::ClientToServer(SinglePlayerUpdate {
        transform: *transform,
    });

    let payload = bincode::serialize(&player_update).unwrap();

    client.send_message(RenetChannel::PlayerToServerSync, payload);
}

fn receive_client_to_server_sync(
    mut server: ResMut<RenetServer>,
    mut player_query: Query<&mut Transform>,
    lobby: Res<Lobby>,
) {
    for (id, entity) in lobby.get_map() {
        if let Some(packet) = server.receive_message(*id, RenetChannel::PlayerToServerSync) {
            if let Ok(deserialized) = bincode::deserialize(&packet).map_err(|e| warn!("{}", e)) {
                match deserialized {
                    PlayerSyncPacket::ClientToServer(single_player_update) => {
                        *player_query.get_mut(*entity).unwrap() = single_player_update.transform;
                    }
                    _ => {
                        warn!("Incorrect packet received from client")
                    }
                }
            }
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

    let serialized = bincode::serialize(&PlayerSyncPacket::ServerToClient(player_update)).unwrap();
    server.broadcast_message(RenetChannel::PlayerToServerSync, serialized);
}

fn receive_server_to_client_sync(
    mut client: ResMut<RenetClient>,
    client_id: Res<ClientID>,
    lobby: Res<Lobby>,
    mut player_query: Query<&mut Transform>,
) {
    if let Some(msg) = client.receive_message(RenetChannel::PlayerToServerSync) {
        let deserialized: PlayerSyncPacket = bincode::deserialize(&msg).unwrap();
        let updated_content = match deserialized {
            ServerToClient(update) => update,
            _ => {
                warn!("Received wrong packet");
                return;
            }
        };

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
