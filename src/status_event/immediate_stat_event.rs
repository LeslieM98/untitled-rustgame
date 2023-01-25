use crate::status_event::stats::*;
use crate::status_event::TargetAssociation;
use bevy::prelude::{Commands, Component, Entity, Query, SystemSet, With};
use std::fmt::{Debug, Formatter};

pub fn get_system_set() -> SystemSet {
    SystemSet::new().with_system(resolve_immediate_stat_events)
}

#[derive(Component, Default)]
pub struct ImmediateStatEventQueue {
    pub events: Vec<ImmediateStatEvent>,
}

#[derive(Clone, Copy)]
pub struct ImmediateStatEvent {
    pub target_association: TargetAssociation,
    ///First stat struct is the source, second is the target
    pub apply: fn(&Stats, &mut Stats),
}

impl Debug for ImmediateStatEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImmediateStatEvent({:?})", self.target_association)
    }
}

pub fn resolve_immediate_stat_events(
    affected_query: Query<(Entity, &ImmediateStatEventQueue)>,
    mut stats_query: Query<&mut Stats>,
    mut commands: Commands,
) {
    for (entity, event_queues) in affected_query.iter() {
        for event in event_queues.events.iter() {
            let source_stats = stats_query
                .get(event.target_association.source)
                .expect(
                    format!("Cannot find source: {:?}", event.target_association.source).as_str(),
                )
                .clone();

            let mut target_stats = stats_query.get_mut(event.target_association.target).expect(
                format!("Cannot find target: {:?}", event.target_association.target).as_str(),
            );
            (event.apply)(&source_stats, &mut target_stats);
        }
        commands
            .entity(entity)
            .insert(ImmediateStatEventQueue::default());
    }
}
