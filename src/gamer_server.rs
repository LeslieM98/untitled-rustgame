use crate::actor::npc::EnemyPlugin;
use crate::network::server::ServerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;

pub struct GameServer;

impl Plugin for GameServer {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins)
            .add_plugin(LogPlugin::default())
            .add_plugin(ServerPlugin::new("127.0.0.1", 42069))
            .add_plugin(EnemyPlugin);
    }
}
