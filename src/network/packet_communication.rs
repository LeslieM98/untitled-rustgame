use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::{DefaultChannel, RenetClient, RenetServer};
use serde::{Deserialize, Serialize};

use super::{client::ClientID, lobby::Lobby};

pub struct NetworkProtocolServerPlugin;
pub struct NetworkProtocolClientPlugin;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum Sender {
    Server,
    Client(u64),
}

impl Plugin for NetworkProtocolServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clear_messages.in_base_set(CoreSet::First))
            .add_system(server_recv_packet)
            .insert_resource(ReceivedMessages::default());
    }
}

impl Plugin for NetworkProtocolClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clear_messages.in_base_set(CoreSet::First))
            .insert_resource(ReceivedMessages::default());
    }
}

pub trait PacketMetaData: 'static + Sync + Send {
    fn get_packet_type() -> PacketType;
    fn get_content_size(&self) -> u128;
}

pub trait BroadcastPacket: PacketMetaData {}
trait TargetedPacket: PacketMetaData {
    fn get_target_client_id() -> u64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PacketType {
    ClientToServerPlayerSync,
    ServerToClientPlayerSync,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Packet {
    pub protocol_version: u16,
    pub packet_type: PacketType,
    pub content_size: u128,
    pub sender: Sender,
    pub content: Vec<u8>,
}

impl Packet {
    fn new<T>(content: &T, sender: Sender) -> Packet
    where
        T: PacketMetaData + Serialize,
    {
        let serialized_content = bincode::serialize(&content).unwrap();
        Packet {
            protocol_version: Self::current_protocol_version(),
            packet_type: T::get_packet_type(),
            content_size: serialized_content.len() as u128,
            sender,
            content: serialized_content,
        }
    }

    fn new_server_packet<T>(content: &T) -> Packet
    where
        T: PacketMetaData + Serialize,
    {
        Self::new(content, Sender::Server)
    }

    fn current_protocol_version() -> u16 {
        0
    }
}

#[derive(Resource, Debug, Default)]
pub struct ReceivedMessages {
    pub recv: HashMap<PacketType, Vec<Packet>>,
}

pub fn client_send_packet<T>(
    mut connection: ResMut<RenetClient>,
    mut content_events: EventReader<T>,
    client_id: Res<ClientID>,
) where
    T: PacketMetaData + Serialize,
{
    for content in content_events.iter() {
        if !connection.can_send_message(DefaultChannel::Unreliable) {
            return;
        }

        let packet = Packet::new(content, Sender::Client(client_id.id));
        if let Ok(serialized) = bincode::serialize(&packet).map_err(|err| warn!("{}", err)) {
            connection.send_message(DefaultChannel::Unreliable, serialized);
        }
    }
}

fn server_recv_packet(
    mut connection: ResMut<RenetServer>,
    lobby: Res<Lobby>,
    mut received_messages: ResMut<ReceivedMessages>,
) {
    for (id, _) in lobby.get_map().iter() {
        while let Some(recv) = connection.receive_message(*id, DefaultChannel::Unreliable) {
            if let Ok(deserialized) =
                bincode::deserialize::<Packet>(&recv).map_err(|err| warn!("{}", err))
            {
                if deserialized.protocol_version != Packet::current_protocol_version() {
                    warn!("Received packet with wrong version, dropping packet");
                    continue;
                }

                if !received_messages
                    .recv
                    .contains_key(&deserialized.packet_type)
                {
                    received_messages
                        .recv
                        .insert(deserialized.packet_type, Vec::new());
                }

                received_messages
                    .recv
                    .get_mut(&deserialized.packet_type)
                    .unwrap()
                    .push(deserialized);
            }
        }
    }
}

pub fn client_recv_packet(
    connection: &mut RenetClient,
    mut received_messages: ResMut<ReceivedMessages>,
) {
    while let Some(recv) = connection.receive_message(DefaultChannel::Unreliable) {
        if let Ok(deserialized) =
            bincode::deserialize::<Packet>(&recv).map_err(|err| warn!("{}", err))
        {
            if deserialized.protocol_version != Packet::current_protocol_version() {
                warn!("Received packet with wrong version, dropping packet");
                continue;
            }

            if !received_messages
                .recv
                .contains_key(&deserialized.packet_type)
            {
                received_messages
                    .recv
                    .insert(deserialized.packet_type, Vec::new());
            }

            received_messages
                .recv
                .get_mut(&deserialized.packet_type)
                .unwrap()
                .push(deserialized);
        }
    }
}

pub fn server_broadcast_packet<T>(connection: &mut RenetServer, content_events: &mut EventReader<T>)
where
    T: BroadcastPacket + Serialize,
{
    for content in content_events {
        if let Ok(serialized) =
            bincode::serialize(&Packet::new_server_packet(content)).map_err(|err| warn!("{}", err))
        {
            connection.broadcast_message(DefaultChannel::Unreliable, serialized);
        }
    }
}

fn clear_messages(mut recv_messages: ResMut<ReceivedMessages>) {
    recv_messages.recv.clear();
}
