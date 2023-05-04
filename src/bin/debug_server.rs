use bevy::prelude::*;
use rust_game::{actor::npc::Enemy, gamer_server::GameServer};

fn main() {
    App::new()
        .add_plugin(GameServer)
        .add_startup_system(spawn_enemies)
        .run();
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
