use crate::actor::health::BaseHealth;
use crate::actor::Actor;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn);
    }
}

#[derive(Component)]
pub struct EnemyMarker;

#[derive(Bundle)]
pub struct Enemy {
    pub actor: Actor,
    pub marker: EnemyMarker,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            actor: Actor::default(),
            marker: EnemyMarker,
        }
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tex_handle = asset_server.load("PNG/Red/texture_04.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tex_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    });

    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    };
    let enemy = Enemy {
        actor: Actor { pbr, ..default() },
        ..default()
    };

    commands.spawn(enemy).insert(BaseHealth::default());
}
