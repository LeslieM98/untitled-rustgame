use crate::network::server::MAX_CONNECTIONS;
use bevy::prelude::{Commands, Entity, Resource};
use bevy::utils::HashMap;

type SpawnFunction = fn(&mut Commands, u64) -> Entity;

#[derive(Resource)]
pub struct Lobby {
    player_ids: HashMap<u64, Entity>,
    spawn_player: SpawnFunction,
}

pub struct SyncConnectedPlayersPackage {
    ids: [Option<u64>; MAX_CONNECTIONS],
}

impl Lobby {
    pub fn new(instantiate_player: SpawnFunction) -> Lobby {
        Lobby {
            spawn_player: instantiate_player,
            player_ids: HashMap::default(),
        }
    }

    pub fn with_player_ids(player_ids: HashMap<u64, Entity>, spawn_player: SpawnFunction) -> Lobby {
        Lobby {
            player_ids,
            spawn_player,
        }
    }

    pub fn generate_sync_package(&self) -> SyncConnectedPlayersPackage {
        let mut data: [Option<u64>; MAX_CONNECTIONS] = [None; MAX_CONNECTIONS];
        for (i, (id, _)) in self.player_ids.iter().enumerate() {
            data[i] = Some(*id);
        }

        SyncConnectedPlayersPackage { ids: data }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_function(_commands: &mut Commands, _player: u64) -> Entity {
        panic!("This should not ever be called")
    }

    #[test]
    fn correct_sync_package() {
        let mut player_ids: HashMap<u64, Entity> = HashMap::default();
        player_ids.insert(0, Entity::from_bits(0));
        player_ids.insert(1, Entity::from_bits(1));
        player_ids.insert(15, Entity::from_bits(25634));

        let lobby = Lobby::with_player_ids(player_ids.clone(), dummy_function);

        let sync_package = lobby.generate_sync_package();

        assert_eq!(sync_package.ids.len(), MAX_CONNECTIONS);

        let sync_ids: Vec<u64> = sync_package
            .ids
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        assert_eq!(sync_ids.len(), player_ids.len());

        for sync_id in sync_ids {
            assert!(player_ids.contains_key(&sync_id));
        }

        let lobby = Lobby::new(dummy_function);
        let sync_package = lobby.generate_sync_package();
        for player in sync_package.ids {
            assert!(player.is_none())
        }
    }
}
