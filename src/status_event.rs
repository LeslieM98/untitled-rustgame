use crate::actor::status::Stats;
use bevy::app::App;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use std::ops::Deref;

const TICK_RATE: f64 = 64.0;

pub struct StatusEventPlugin;
impl Plugin for StatusEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init)
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(resolve_events)
                    .with_run_criteria(FixedTimestep::steps_per_second(TICK_RATE)),
            );
    }
}

#[derive(Component, Default)]
pub struct ActionReceivedEventQueue {
    pub events: Vec<ActionReceivedEvent>,
}

pub struct ActionReceivedEvent {
    pub apply: Box<dyn Fn(&mut Stats) -> () + Sync + Send>,
}

fn resolve_events(
    mut affected_query: Query<(Entity, &mut Stats, &ActionReceivedEventQueue)>,
    mut commands: Commands,
) {
    for (entity, mut stats, event_queue) in affected_query.iter_mut() {
        for event in &event_queue.events {
            event.apply.deref()(&mut stats);
        }
        commands
            .entity(entity)
            .insert(ActionReceivedEventQueue::default());
    }
}

fn init(mut commands: Commands, health_queries: Query<Entity, With<Stats>>) {
    for entity in &health_queries {
        commands
            .entity(entity)
            .insert(ActionReceivedEventQueue::default());
    }
}
