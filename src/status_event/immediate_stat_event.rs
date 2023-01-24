use crate::status_event::Stats::*;
use crate::status_event::TargetAssociation;
use bevy::prelude::{Commands, Component, Entity, Query, SystemSet, With};
use std::fmt::{write, Debug, Formatter};

pub fn get_system_set() -> SystemSet {
    SystemSet::new().with_system(resolve_immediate_stat_events)
}

#[derive(Component, Default)]
pub struct HealthEventQueue {
    pub events: Vec<ImmediateStatEvent>,
}

#[derive(Clone, Copy)]
pub struct ImmediateStatEvent {
    pub target_association: TargetAssociation,
    ///First stat struct is the source, second is the target
    pub apply: fn(&Stats, &Stats) -> Stats,
}

impl Debug for ImmediateStatEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImmediateStatEvent({:?})", self.target_association)
    }
}

pub fn resolve_immediate_stat_events(
    affected_query: Query<(Entity, &HealthEventQueue)>,
    mut stats_query: Query<&mut Stats>,
    mut commands: Commands,
) {
    for (entity, event_queues) in affected_query.iter() {
        for event in event_queues.events.iter() {
            let source_stats = stats_query.get(event.target_association.source).expect(
                format!("Cannot find source: {:?}", event.target_association.source).as_str(),
            );

            let target_stats = stats_query.get(event.target_association.target).expect(
                format!("Cannot find target: {:?}", event.target_association.target).as_str(),
            );

            let target_delta = (event.apply)(source_stats, target_stats);

            let mut target_stats = stats_query.get_mut(event.target_association.target).expect(
                format!("Cannot find target: {:?}", event.target_association.target).as_str(),
            );

            target_stats.apply_delta(&target_delta);
        }
        commands.entity(entity).insert(HealthEventQueue::default());
    }
}

pub fn init(mut commands: Commands, health_queries: Query<Entity, With<Stats>>) {
    for entity in &health_queries {
        commands.entity(entity).insert(HealthEventQueue::default());
    }
}
