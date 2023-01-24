pub mod immediate_stat_event;
pub mod ticking_stat_event;

use bevy::app::App;
use bevy::prelude::*;
use bevy::time::FixedTimestep;

const TICK_RATE: f64 = 64.0;

pub struct StatusEventPlugin;
impl Plugin for StatusEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, immediate_stat_event::init)
            .add_startup_system_to_stage(StartupStage::PostStartup, ticking_stat_event::init)
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                immediate_stat_event::get_system_set()
                    .with_run_criteria(FixedTimestep::steps_per_second(TICK_RATE)),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                ticking_stat_event::get_system_set()
                    .with_run_criteria(FixedTimestep::steps_per_second(TICK_RATE)),
            );
    }
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
