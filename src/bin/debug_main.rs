use bevy::prelude::*;
use rust_game::actor::npc::Enemy;
use rust_game::debug::DebugPlugin;
use std::env;
use rust_game::game::Game;

struct DebugScene;

impl Plugin for DebugScene {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, rust_game::load_debug_scene)
            .add_systems(Startup,spawn_enemies);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let enemy1 = Enemy::from_pos(
        Transform::from_xyz(3.0, 1.0, 0.0),
        &mut meshes,
        &asset_server,
        &mut materials,
    );

    commands.spawn(enemy1);
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
