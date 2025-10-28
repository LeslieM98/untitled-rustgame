use bevy::prelude::*;
use untitled_rustgame::debug::DebugPlugin;
use untitled_rustgame::Game;

fn main() {
    let mut app = App::new();
    app.add_plugins(Game)
        .add_plugins(DebugPlugin);
    app.run();
}

