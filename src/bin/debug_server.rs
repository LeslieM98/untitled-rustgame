use bevy::log::LogPlugin;
use bevy::prelude::*;
use rust_game::network::server::ServerPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::new("localhost", 42069))
        .run();
}
