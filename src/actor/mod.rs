pub mod enemy;
pub mod player;
pub mod status;
pub mod target;

use std::time::SystemTime;

use crate::actor::status::Stats;
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

#[derive(Bundle, Default)]
pub struct Actor {
    pub name: Name,
    pub pbr: PbrBundle,
    pub stats: Stats,
    pickable: PickableBundle,
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
