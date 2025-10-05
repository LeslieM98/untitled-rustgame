use bevy::prelude::*;
use rust_game::debug::DebugPlugin;
use std::env;
use rust_game::game::Game;

struct DebugScene;

impl Plugin for DebugScene {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, rust_game::load_debug_scene);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(Game)
        .add_plugins(DebugScene);

    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug-editor")) {
        app.add_plugins(DebugPlugin);
    }
    app.run();
}
