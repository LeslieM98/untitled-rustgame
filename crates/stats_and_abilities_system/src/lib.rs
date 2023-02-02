pub mod event_dispatcher;
mod stats;
use bevy::prelude::*;
use event_dispatcher::EventDispatcher;

type TickRateType = u32;

#[derive(StageLabel, Clone, Copy)]
enum StatAbilityStage {
    ClearDispatcher,
}

pub struct StatAbilityPlugin {
    tick_rate: u32,
}

impl Plugin for StatAbilityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EventDispatcher::default())
            .add_stage_after(
                CoreStage::PostUpdate,
                StatAbilityStage::ClearDispatcher,
                SystemStage::parallel(),
            );
    }
}

impl StatAbilityPlugin {
    /// Tickrate describes how many ticks per second happen
    pub fn new(tick_rate: TickRateType) -> Self {
        Self { tick_rate }
    }
}
