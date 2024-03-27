mod ai;

use crate::actor::npc::ai::AIPlugin;
use crate::actor::Actor;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AIPlugin);
    }
}

#[derive(Component)]
pub struct NPCMarker;

#[derive(Bundle)]
pub struct Enemy {
    pub actor: Actor,
    pub marker: NPCMarker,
}

impl Enemy {
    pub fn from_pos(
        pos: Transform,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Enemy {
        let tex_handle = asset_server.load("PNG/Red/texture_04.png");
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(tex_handle.clone()),
            unlit: false,
            ..default()
        });

        let pbr = PbrBundle {
            mesh: meshes.add(Mesh::from(Capsule3d::default())),
            material: material_handle,
            transform: pos,
            ..default()
        };
        Enemy {
            actor: Actor { pbr, ..default() },
            ..default()
        }
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            actor: Actor::default(),
            marker: NPCMarker,
        }
    }
}
