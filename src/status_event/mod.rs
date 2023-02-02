pub mod stats;

use bevy::app::App;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

const TICK_RATE: f64 = 64.0;

pub struct StatusEventPlugin;
impl Plugin for StatusEventPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Copy, Clone, Debug)]
pub struct TargetAssociation {
    pub source: Entity,
    pub target: Entity,
}

impl TargetAssociation {
    pub fn new(source: Entity, target: Entity) -> TargetAssociation {
        Self { source, target }
    }
}
