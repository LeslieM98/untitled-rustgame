use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_renet::renet::{DefaultChannel, RenetClient, RenetServer};
use serde::{Deserialize, Serialize};

use crate::network::server::MAX_CONNECTIONS;
use crate::network::server::{ClientConnectedEvent, ClientDisconnectedEvent};

use super::client::ClientID;
use super::packet_communication::{Packet, PacketMetaData, PacketType, Sender, Target};
use super::remote_player::{spawn_remote_player, PlayerID};

type SpawnFunction = Box<dyn Fn(&mut Commands, u64) -> Entity + Send + Sync>;

#[derive(Default)]
pub struct LobbyClientPlugin;

impl Plugin for LobbyClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lobby::default())
            .add_event::<AttachModelToPlayerEvent>()
            .add_event::<LobbySync>()
            .add_system(client_apply_sync)
            .add_system(client_attach_model_to_player)
            .add_system(client_recv_sync);
    }
}

#[derive(Default)]
pub struct LobbyServerPlugin;

impl Plugin for LobbyServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lobby::default())
            .add_system(server_client_connected)
            .add_system(server_client_disconnected)
            .add_system(
                server_send_sync
                    .after(server_client_connected)
                    .after(server_client_disconnected)
                    .run_if(|lobby: Res<Lobby>| lobby.is_changed()),
            );
    }
}

#[derive(Resource, Default, Debug)]
pub struct Lobby {
    connected_clients: HashMap<u64, Entity>,
}

impl Lobby {
    pub fn get_map(&self) -> &HashMap<u64, Entity> {
        &self.connected_clients
    }

    pub fn generate_sync_packet(&self) -> LobbySync {
        let mut lobby_sync = LobbySync::default();
        for (i, (id, _)) in self.connected_clients.iter().enumerate() {
            lobby_sync.connected_clients[i] = Some(*id);
        }
        return lobby_sync;
    }

    fn disconnect_clients(&mut self, sync: &LobbySync) -> Vec<Entity> {
        let mut disconnected_entities = Vec::new();
        let mut disconnected_ids = Vec::new();
        for (id, entity) in &self.connected_clients {
            if !sync.connected_clients.contains(&Some(*id)) {
                disconnected_entities.push(*entity);
                disconnected_ids.push(*id);
            }
        }
        for id in disconnected_ids {
            self.connected_clients.remove(&id);
        }
        return disconnected_entities;
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LobbySync {
    pub connected_clients: [Option<u64>; MAX_CONNECTIONS],
}

impl PacketMetaData for LobbySync {
    fn get_packet_type() -> PacketType {
        PacketType::LobbySync
    }

    fn get_content_size(&self) -> u128 {
        bincode::serialized_size(self).unwrap().into()
    }
}

struct AttachModelToPlayerEvent {
    id: u64,
}

fn server_client_connected(
    mut lobby: ResMut<Lobby>,
    mut client_connected_events: EventReader<ClientConnectedEvent>,
    mut commands: Commands,
) {
    for event in client_connected_events.iter() {
        let entity = commands
            .spawn(TransformBundle::default())
            .insert(PlayerID::new(event.id))
            .id();
        lobby.connected_clients.insert(event.id, entity);
    }
}

fn server_client_disconnected(
    mut lobby: ResMut<Lobby>,
    mut client_disconnected_events: EventReader<ClientDisconnectedEvent>,
    mut commands: Commands,
) {
    for event in client_disconnected_events.iter() {
        let entity = lobby.connected_clients.get(&event.id);
        if let Some(client_entity) = entity {
            commands.entity(*client_entity).despawn();
        }
        lobby.connected_clients.remove(&event.id);
    }
}

fn server_send_sync(lobby: Res<Lobby>, mut server: ResMut<RenetServer>) {
    let sync_packet = Packet::new(
        &lobby.generate_sync_packet(),
        Sender::Server,
        Target::Broadcast,
    );
    let serialized = bincode::serialize(&sync_packet).unwrap();

    server.broadcast_message(DefaultChannel::Reliable, serialized);
}

fn client_recv_sync(mut client: ResMut<RenetClient>, mut event_writer: EventWriter<LobbySync>) {
    while let Some(recv) = client.receive_message(DefaultChannel::Reliable) {
        let packet = bincode::deserialize::<Packet>(&recv).unwrap();
        let sync = bincode::deserialize::<LobbySync>(&packet.content).unwrap();

        event_writer.send(sync);
    }
}

fn client_apply_sync(
    mut lobby: ResMut<Lobby>,
    mut event_reader: EventReader<LobbySync>,
    mut commands: Commands,
    mut model_events: EventWriter<AttachModelToPlayerEvent>,
    client_id: Res<ClientID>,
) {
    for sync in event_reader.iter() {
        for entity in lobby.disconnect_clients(&sync).iter() {
            commands.entity(*entity).despawn_recursive();
        }
        for client in sync.connected_clients {
            if let Some(id) = client {
                if client_id.id == id {
                    continue;
                }
                if !lobby.connected_clients.contains_key(&id) {
                    lobby
                        .connected_clients
                        .insert(id, spawn_remote_player(&mut commands, id));
                    model_events.send(AttachModelToPlayerEvent { id })
                }
            }
        }
    }
}

fn client_attach_model_to_player(
    lobby: Res<Lobby>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<AttachModelToPlayerEvent>,
) {
    if !event_reader.is_empty() {
        let player_model = asset_server.load("glTF/base model/base_model.gltf#Scene0");
        for event in event_reader.iter() {
            lobby.connected_clients.get(&event.id).map(|x| {
                commands.entity(*x).insert(SceneBundle {
                    scene: player_model.clone(),
                    ..default()
                });
            });
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sync_package() {
        let mut lobby = Lobby::default();
        lobby.connected_clients.insert(0, Entity::from_bits(0));
        lobby.connected_clients.insert(420, Entity::from_bits(1));
        lobby.connected_clients.insert(69, Entity::from_bits(2));

        let subject = lobby.generate_sync_packet();

        assert_eq!(subject.connected_clients.iter().flatten().count(), 3);
        assert!(subject.connected_clients.contains(&Some(0)));
        assert!(subject.connected_clients.contains(&Some(420)));
        assert!(subject.connected_clients.contains(&Some(69)));
    }

    #[test]
    fn simulate_connecting_player_on_server_side() {
        let mut server = App::new();
        server
            .insert_resource(Lobby::default())
            .add_system(server_client_connected)
            .add_system(server_client_disconnected)
            .add_event::<ClientConnectedEvent>()
            .add_event::<ClientDisconnectedEvent>();
        server.update();
        let entity_count = server.world.entities().len();

        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .is_empty());

        server.world.send_event(ClientConnectedEvent { id: 42069 });
        server.update();

        let lobby = server.world.get_resource::<Lobby>().unwrap();
        assert!(!lobby.connected_clients.is_empty());
        assert!(lobby.connected_clients.contains_key(&42069));
        assert_eq!(server.world.entities().len(), entity_count + 1);

        server.world.send_event(ClientConnectedEvent { id: 420 });
        server.update();

        let lobby = server.world.get_resource::<Lobby>().unwrap();
        assert!(!lobby.connected_clients.is_empty());
        assert!(lobby.connected_clients.contains_key(&420));
        assert_eq!(server.world.entities().len(), entity_count + 2);

        server.world.send_event(ClientConnectedEvent { id: 69 });
        server.update();

        let lobby = server.world.get_resource::<Lobby>().unwrap();
        assert!(!lobby.connected_clients.is_empty());
        assert!(lobby.connected_clients.contains_key(&69));
        assert_eq!(server.world.entities().len(), entity_count + 3);
    }

    #[test]
    fn simulate_connecting_player_on_client_side() {
        let mut server_lobby = Lobby::default();
        server_lobby
            .connected_clients
            .insert(42069, Entity::from_bits(0));
        let mut client = App::new();
        client
            .add_event::<AttachModelToPlayerEvent>()
            .add_event::<LobbySync>()
            .insert_resource(Lobby::default())
            .add_system(client_apply_sync);
        client.update();

        let lobby = client.world.get_resource::<Lobby>().unwrap();
        assert_eq!(lobby.connected_clients.len(), 0);

        let mock_package = server_lobby.generate_sync_packet();
        client.world.send_event(mock_package);

        client.update();
        let lobby = client.world.get_resource::<Lobby>().unwrap();

        assert_eq!(lobby.connected_clients.len(), 1);
        assert!(lobby.connected_clients.contains_key(&42069));

        server_lobby
            .connected_clients
            .insert(420, Entity::from_bits(0));
        let lobby_sync = server_lobby.generate_sync_packet();
        client.world.send_event(lobby_sync);

        client.update();
        let lobby = client.world.get_resource::<Lobby>().unwrap();

        assert_eq!(lobby.connected_clients.len(), 2);
        assert!(lobby.connected_clients.contains_key(&420));

        server_lobby
            .connected_clients
            .insert(69, Entity::from_bits(0));
        let lobby_sync = server_lobby.generate_sync_packet();
        client.world.send_event(lobby_sync);

        client.update();
        let lobby = client.world.get_resource::<Lobby>().unwrap();

        assert_eq!(lobby.connected_clients.len(), 3);
        assert!(lobby.connected_clients.contains_key(&69));
    }

    #[test]
    fn simulate_disconnecting_player_on_server_side() {
        let mut server = App::new();
        server
            .insert_resource(Lobby::default())
            .add_system(server_client_connected)
            .add_system(server_client_disconnected)
            .add_event::<ClientConnectedEvent>()
            .add_event::<ClientDisconnectedEvent>();
        server.update();
        server.world.send_event(ClientConnectedEvent { id: 42069 });
        server.update();
        server.world.send_event(ClientConnectedEvent { id: 69 });
        server.update();
        server.world.send_event(ClientConnectedEvent { id: 420 });
        server.update();

        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            3
        );
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        server.world.send_event(ClientDisconnectedEvent { id: 420 });
        server.update();
        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            2
        );
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        server.world.send_event(ClientDisconnectedEvent { id: 69 });
        server.update();
        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            1
        );
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        server
            .world
            .send_event(ClientDisconnectedEvent { id: 42069 });
        server.update();
        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            0
        );
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));
    }

    #[test]
    fn simulate_disconnecting_player_on_client_side() {
        let mut server = App::new();
        server
            .insert_resource(Lobby::default())
            .add_system(server_client_connected)
            .add_system(server_client_disconnected)
            .add_event::<ClientConnectedEvent>()
            .add_event::<ClientDisconnectedEvent>();

        let mut client = App::new();
        client
            .add_event::<AttachModelToPlayerEvent>()
            .add_event::<LobbySync>()
            .insert_resource(Lobby::default())
            .add_system(client_apply_sync);
        client.update();

        server.update();
        server.world.send_event(ClientConnectedEvent { id: 42069 });
        server.update();
        server.world.send_event(ClientConnectedEvent { id: 69 });
        server.update();
        server.world.send_event(ClientConnectedEvent { id: 420 });
        server.update();

        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            3
        );
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        let lobby_sync = server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .generate_sync_packet();
        client.world.send_event(lobby_sync);

        client.update();

        assert_eq!(
            client
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            3
        );
        assert!(client
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(client
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(client
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        server.world.send_event(ClientDisconnectedEvent { id: 420 });
        server.update();
        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            2
        );
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        let lobby_sync = server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .generate_sync_packet();
        client.world.send_event(lobby_sync);
        client.update();

        let client_lobby = client.world.get_resource::<Lobby>().unwrap();

        assert_eq!(client_lobby.connected_clients.len(), 2);
        assert!(client_lobby.connected_clients.contains_key(&42069));
        assert!(client_lobby.connected_clients.contains_key(&69));
        assert!(!client_lobby.connected_clients.contains_key(&420));

        server.world.send_event(ClientDisconnectedEvent { id: 69 });
        server.update();
        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            1
        );
        assert!(server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        let lobby_sync = server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .generate_sync_packet();
        client.world.send_event(lobby_sync);
        client.update();

        let client_lobby = client.world.get_resource::<Lobby>().unwrap();
        assert_eq!(client_lobby.connected_clients.len(), 1);
        assert!(client_lobby.connected_clients.contains_key(&42069));
        assert!(!client_lobby.connected_clients.contains_key(&69));
        assert!(!client_lobby.connected_clients.contains_key(&420));

        server
            .world
            .send_event(ClientDisconnectedEvent { id: 42069 });
        server.update();
        assert_eq!(
            server
                .world
                .get_resource::<Lobby>()
                .unwrap()
                .connected_clients
                .len(),
            0
        );
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&42069));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&69));
        assert!(!server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .connected_clients
            .contains_key(&420));

        let lobby_sync = server
            .world
            .get_resource::<Lobby>()
            .unwrap()
            .generate_sync_packet();
        client.world.send_event(lobby_sync);
        client.update();

        let client_lobby = client.world.get_resource::<Lobby>().unwrap();
        assert_eq!(client_lobby.connected_clients.len(), 0);
        assert!(!client_lobby.connected_clients.contains_key(&42069));
        assert!(!client_lobby.connected_clients.contains_key(&69));
        assert!(!client_lobby.connected_clients.contains_key(&420));
    }
}
