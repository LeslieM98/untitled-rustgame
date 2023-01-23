pub mod npc;
pub mod player;
pub mod status;
pub mod target;

use std::time::SystemTime;

use crate::actor::status::Stats;
use crate::actor::target::{Target, Targetable};
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

#[derive(Component)]
pub enum CombatStatus {
    InCombat,
    OutOfCombat,
}

impl Default for CombatStatus {
    fn default() -> Self {
        Self::OutOfCombat
    }
}

#[derive(Component)]
pub enum Relationship {
    Enemy,
    Neutral,
    Friend,
}

impl Default for Relationship {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Bundle, Default)]
pub struct Actor {
    pub name: Name,
    pub pbr: PbrBundle,
    pub stats: Stats,
    pub combat_status: CombatStatus,
    pub target: Target,
    targetable: Targetable,
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
