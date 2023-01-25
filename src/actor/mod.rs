pub mod npc;
pub mod player;
pub mod target;

use crate::actor::target::{Target, Targetable};
use crate::status_event::immediate_stat_event::{ImmediateStatEvent, ImmediateStatEventQueue};
use crate::status_event::stats::*;
use crate::status_event::ticking_stat_event::TickingStatEventQueue;
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
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

#[derive(Bundle, Default)]
pub struct Actor {
    pub name: Name,
    pub pbr: PbrBundle,
    pub stats: Stats,
    pub combat_status: CombatStatus,
    pub target: Target,
    pub ticking_stat_event_queue: TickingStatEventQueue,
    pub immediate_stat_event_queue: ImmediateStatEventQueue,
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
