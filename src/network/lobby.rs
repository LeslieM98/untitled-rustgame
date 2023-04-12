use crate::network::server::MAX_CONNECTIONS;
use bevy::prelude::{Commands, Entity, Resource};
use bevy::utils::HashMap;

type SpawnFunction = Box<dyn Fn(&mut Commands, u64) -> Entity + Send + Sync>;

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

    fn register_client(&mut self, client_id: u64, commands: &mut Commands) -> Entity {
        let entity = (self.spawn_player)(commands, client_id);
        self.player_ids.insert(client_id, entity);
        return *self.player_ids.get(&client_id).unwrap();
    }

    fn unregister_client(&mut self, client_id: u64, commands: &mut Commands) {
        let entity = self.player_ids.remove(&client_id);
        entity.iter().for_each(|x| commands.entity(*x).despawn());
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
        mut commands: &mut Commands,
    ) {
        let newly_connected_players: Vec<&u64> = payload
            .iter()
            .flatten()
            .filter(|x| !self.player_ids.contains_key(x))
            .collect();

        for player in newly_connected_players {
            let entity = (self.spawn_player)(&mut commands, *player);
            self.player_ids.insert(*player, entity);
        }
    }

    pub fn apply_sync_package(
        &mut self,
        payload: &[Option<u64>; MAX_CONNECTIONS],
        mut commands: &mut Commands,
    ) {
        self.handle_disconnected_players(payload, commands);
        self.handle_newly_connected_players(payload, commands);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::CommandQueue;
    use bevy::prelude::{VisibilityBundle, World};

    fn dummy_function(_commands: &mut Commands, _player: u64) -> Entity {
        panic!("This should not ever be called")
    }

    #[test]
    fn correct_sync_package() {
        let mut player_ids: HashMap<u64, Entity> = HashMap::default();
        player_ids.insert(0, Entity::from_bits(0));
        player_ids.insert(1, Entity::from_bits(1));
        player_ids.insert(15, Entity::from_bits(25634));

        let lobby =
            Lobby::with_player_ids(player_ids.clone(), Box::new(|x, y| dummy_function(x, y)));

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

        let lobby = Lobby::new(Box::new(|x, y| dummy_function(x, y)));
        let sync_package = lobby.generate_sync_package();
        for player in sync_package.ids {
            assert!(player.is_none())
        }
    }

    #[test]
    fn correct_player_connection_sync() {
        static mut SERVER_COUNTER: u64 = 0;
        let server_lambda: SpawnFunction = Box::new(|_, _| unsafe {
            SERVER_COUNTER += 1;
            Entity::from_bits(SERVER_COUNTER)
        });
        let mut server_lobby = Lobby::new(server_lambda);

        static mut CLIENT_COUNTER: u64 = 0;
        let client_lambda: SpawnFunction = Box::new(|_, _| unsafe {
            CLIENT_COUNTER += 1;
            Entity::from_bits(CLIENT_COUNTER)
        });
        let mut client_lobby = Lobby::new(client_lambda);

        let mut commandq = CommandQueue::default();
        let world = World::new();
        let mut commands = Commands::new(&mut commandq, &world);

        unsafe {
            assert_eq!(0, SERVER_COUNTER);
            assert_eq!(0, CLIENT_COUNTER);
            assert_eq!(0, server_lobby.player_ids.len());
            assert_eq!(0, client_lobby.player_ids.len());
        }
        server_lobby.register_client(42069, &mut commands);

        unsafe {
            assert_eq!(1, SERVER_COUNTER);
            assert_eq!(0, CLIENT_COUNTER);
            assert_eq!(1, server_lobby.player_ids.len());
            assert_eq!(0, client_lobby.player_ids.len());
        }

        let sync_package = server_lobby.generate_sync_package();
        client_lobby.apply_sync_package(&sync_package.ids, &mut commands);

        unsafe {
            assert_eq!(1, SERVER_COUNTER);
            assert_eq!(1, CLIENT_COUNTER);
            assert_eq!(1, server_lobby.player_ids.len());
            assert_eq!(1, client_lobby.player_ids.len());
        }

        let sync_package = server_lobby.generate_sync_package();
        client_lobby.apply_sync_package(&sync_package.ids, &mut commands);

        unsafe {
            assert_eq!(1, SERVER_COUNTER);
            assert_eq!(1, CLIENT_COUNTER);
            assert_eq!(1, server_lobby.player_ids.len());
            assert_eq!(1, client_lobby.player_ids.len());
        }

        server_lobby.register_client(69420, &mut commands);

        unsafe {
            assert_eq!(2, SERVER_COUNTER);
            assert_eq!(1, CLIENT_COUNTER);
            assert_eq!(2, server_lobby.player_ids.len());
            assert_eq!(1, client_lobby.player_ids.len());
        }

        let sync_package = server_lobby.generate_sync_package();
        client_lobby.apply_sync_package(&sync_package.ids, &mut commands);

        unsafe {
            assert_eq!(2, SERVER_COUNTER);
            assert_eq!(2, CLIENT_COUNTER);
            assert_eq!(2, server_lobby.player_ids.len());
            assert_eq!(2, client_lobby.player_ids.len());
        }
    }

    #[test]
    fn correct_player_disconnection_sync() {
        let mut server_commandq = CommandQueue::default();
        let server_world = World::new();
        let mut server_commands = Commands::new(&mut server_commandq, &server_world);

        let mut client_commandq = CommandQueue::default();
        let client_world = World::new();
        let mut client_commands = Commands::new(&mut client_commandq, &client_world);

        let mut server_player_ids: HashMap<u64, Entity> = HashMap::default();
        server_player_ids.insert(0, server_commands.spawn(VisibilityBundle::default()).id());
        server_player_ids.insert(1, server_commands.spawn(VisibilityBundle::default()).id());
        server_player_ids.insert(15, server_commands.spawn(VisibilityBundle::default()).id());

        let mut client_player_ids: HashMap<u64, Entity> = HashMap::default();
        client_player_ids.insert(0, client_commands.spawn(VisibilityBundle::default()).id());
        client_player_ids.insert(1, client_commands.spawn(VisibilityBundle::default()).id());
        client_player_ids.insert(15, client_commands.spawn(VisibilityBundle::default()).id());

        let mut server_lobby = Lobby::with_player_ids(
            server_player_ids.clone(),
            Box::new(|x, y| dummy_function(x, y)),
        );
        let mut client_lobby =
            Lobby::with_player_ids(server_player_ids, Box::new(|x, y| dummy_function(x, y)));

        assert_eq!(3, server_lobby.player_ids.len());
        assert_eq!(3, client_lobby.player_ids.len());

        server_lobby.unregister_client(0, &mut server_commands);

        assert_eq!(2, server_lobby.player_ids.len());
        assert_eq!(3, client_lobby.player_ids.len());

        client_lobby.apply_sync_package(
            &server_lobby.generate_sync_package().ids,
            &mut server_commands,
        );

        assert_eq!(2, server_lobby.player_ids.len());
        assert_eq!(2, client_lobby.player_ids.len());
    }
}
