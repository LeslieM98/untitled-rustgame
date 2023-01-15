pub mod enemy;
mod health;
pub mod player;
mod target;

use std::time::SystemTime;

use bevy::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    pub name: Name,
    pub pbr: PbrBundle,
}

#[derive(Component)]
pub struct Name {
    pub value: String,
}

impl Default for Name {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            value: now.to_string(),
        }
    }
}
impl Default for Actor {
    fn default() -> Self {
        Self {
            name: Name::default(),
            pbr: PbrBundle::default(),
        }
    }
}
