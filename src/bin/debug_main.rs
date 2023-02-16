use bevy::prelude::*;
use rust_game::actor::npc::Enemy;
use rust_game::debug::DebugPlugin;
use rust_game::Game;
use std::env;
use std::fmt::Debug;

struct DebugScene;

impl Plugin for DebugScene {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_debug_scene)
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_enemies);
    }
}

fn load_debug_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let my_gltf = asset_server.load("glTF/Debug_Scene.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..Default::default()
    });
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_model = asset_server.load("glTF/base model/base_model.gltf#Scene0");
    rust_game::actor::player::spawn_player(commands, player_model);
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

    let enemy2 = Enemy::from_pos(
        Transform::from_xyz(-3.0, 1.0, 0.0),
        &mut meshes,
        &asset_server,
        &mut materials,
    );

    commands.spawn(enemy1);
    commands.spawn(enemy2);
}

fn main() {
    let mut app = App::new();
    app.add_plugin(Game).add_plugin(DebugScene);

    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug-editor")) {
        app.add_plugin(DebugPlugin);
    }
    app.run();
}
