pub mod player;

use bevy::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    pub pbr: PbrBundle,
}
impl Default for Actor {
    fn default() -> Self {
        Self {
            pbr: PbrBundle::default(),
        }
    }
}
