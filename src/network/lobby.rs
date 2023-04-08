use crate::network::server::MAX_CONNECTIONS;
use bevy::prelude::{Commands, Entity, Resource};
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct Lobby {
    player_ids: HashMap<u64, Entity>,
    spawn_player: fn(&mut Commands, u64) -> Entity,
}

pub enum LobbyPacket {
    SyncConnectedPlayers([Option<u64>; MAX_CONNECTIONS]),
}

impl Lobby {
    pub fn new(instantiate_player: fn(&mut Commands, u64) -> Entity) -> Lobby {
        Lobby {
            spawn_player: instantiate_player,
            player_ids: HashMap::default(),
        }
    }

    pub fn generate_sync_package(&self) -> LobbyPacket {
        let mut data: [Option<u64>; MAX_CONNECTIONS] = [None; MAX_CONNECTIONS];
        for (i, (id, _)) in self.player_ids.iter().enumerate() {
            data[i] = Some(*id);
        }

        LobbyPacket::SyncConnectedPlayers(data)
    }

    fn handle_disconnected_players(
        &mut self,
        payload: &[Option<u64>; MAX_CONNECTIONS],
        commands: &mut Commands,
    ) {
        let disconnected_players: Vec<u64> = self
            .player_ids
            .iter()
            .filter(|(x, _)| !payload.contains(&Some(**x)))
            .map(|(x, _)| *x)
            .collect();

        for disconnected_player in disconnected_players {
            commands
                .entity(*self.player_ids.get(&disconnected_player).unwrap())
                .despawn();

            self.player_ids.remove(&disconnected_player);
        }
    }

    fn handle_newly_connected_players(
        &mut self,
        payload: &[Option<u64>; MAX_CONNECTIONS],
        mut commands: Commands,
    ) {
        let newly_connected_players: Vec<u64> = payload
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .filter(|x| self.player_ids.contains_key(x))
            .collect();

        for player in newly_connected_players {
            let entity = (self.spawn_player)(&mut commands, player);
            self.player_ids.insert(player, entity);
        }
    }

    pub fn apply_sync_package(
        &mut self,
        payload: &[Option<u64>; MAX_CONNECTIONS],
        mut commands: Commands,
    ) {
        self.handle_disconnected_players(payload, &mut commands);
    }
}
