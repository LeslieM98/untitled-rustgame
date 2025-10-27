use bevy::prelude::*;
use untitled_rustgame::debug::DebugPlugin;
use std::env;
use untitled_rustgame::Game;

struct DebugScene;

impl Plugin for DebugScene {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, untitled_rustgame::load_debug_scene);
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
