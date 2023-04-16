use bevy::prelude::{
    info, Commands, Component, Entity, Quat, Query, ResMut, Transform, Vec3, With,
};
use bevy_renet::renet::RenetClient;
use bincode::de::read::Reader;
use bincode::enc::write::Writer;
use bincode::enc::Encoder;
use bincode::error::EncodeError;
use bincode::{Decode, Encode};

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

#[derive(Debug, PartialEq, Clone, Copy)]
struct PlayerUpdatePacket {
    transform: Transform,
}

impl PlayerUpdatePacket {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}

impl Encode for PlayerUpdatePacket {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let writer = encoder.writer();

        let translation_x = self.transform.translation.x.to_le_bytes();
        let translation_y = self.transform.translation.y.to_le_bytes();
        let translation_z = self.transform.translation.z.to_le_bytes();

        writer.write(&translation_x)?;
        writer.write(&translation_y)?;
        writer.write(&translation_z)?;

        let rotation = self.transform.rotation.to_array();
        for element in rotation {
            let encoded = element.to_le_bytes();
            writer.write(&encoded)?;
        }

        let scale_x = self.transform.scale.x.to_le_bytes();
        let scale_y = self.transform.scale.y.to_le_bytes();
        let scale_z = self.transform.scale.z.to_le_bytes();

        writer.write(&scale_x)?;
        writer.write(&scale_y)?;
        writer.write(&scale_z)?;

        Ok(())
    }
}

impl Decode for PlayerUpdatePacket {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let reader = decoder.reader();
        let mut buff = [0; 4];

        reader.read(&mut buff)?;
        let translation_x = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let translation_y = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let translation_z = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let rotation_x = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let rotation_y = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let rotation_z = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let rotation_w = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let scale_x = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let scale_y = f32::from_le_bytes(buff);
        reader.read(&mut buff)?;
        let scale_z = f32::from_le_bytes(buff);

        let transform = Transform {
            translation: Vec3::new(translation_x, translation_y, translation_z),
            rotation: Quat::from_array([rotation_x, rotation_y, rotation_z, rotation_w]),
            scale: Vec3::new(scale_x, scale_y, scale_z),
        };

        Ok(PlayerUpdatePacket::new(transform))
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

    let player_update = PlayerUpdatePacket {
        transform: *transform,
    };

    let payload = bincode::encode_to_vec(player_update, bincode::config::standard())
        .expect("Error while encoding player transform");

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
        let initial = PlayerUpdatePacket::new(transform);

        let data = bincode::encode_to_vec(initial, bincode::config::standard())
            .expect("Error while encoding");

        let (subject, size): (PlayerUpdatePacket, usize) =
            bincode::decode_from_slice(&data, bincode::config::standard())
                .expect("Error while decoding");

        assert_eq!(size, data.len());
        assert_eq!(initial, subject);
    }
}
