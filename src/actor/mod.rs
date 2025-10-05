pub mod player;
pub mod target;

use crate::actor::target::{Target, Targetable};
use bevy::prelude::*;
use std::time::SystemTime;

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

#[derive(Component)]
pub struct Name {
    pub value: String,
}

impl Name {
    pub fn new(value: String) -> Name {
        Name { value }
    }
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

#[derive(Bundle, Default)]
pub struct Actor {
    pub name: Name,
    pub combat_status: CombatStatus,
    pub target: Target,
    targetable: Targetable,
}
