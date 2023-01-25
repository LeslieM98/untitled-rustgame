use crate::status_event::immediate_stat_event::{HealthEventQueue, ImmediateStatEvent};
use crate::status_event::Stats::*;
use bevy::prelude::*;

pub type TickType = u32;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(remove_ticking_events)
        .with_system(resolve_ticking_events)
}

#[derive(Debug)]
pub enum TickDuration {
    Infinite,
    Finite(TickType),
}
#[derive(Debug)]
pub struct TickingStatEvent {
    tick_duration: TickDuration,
    current_tick: TickType,
    tick_trigger_interval: TickType,
    applicable: ImmediateStatEvent,
    marked_for_removal: bool,
}
impl TickingStatEvent {
    pub fn tick(&mut self) -> Option<&ImmediateStatEvent> {
        self.current_tick += 1;
        match &self.tick_duration {
            TickDuration::Infinite => {
                if self.current_tick > self.tick_trigger_interval {
                    self.current_tick = 0;
                }
            }
            TickDuration::Finite(duration) => {
                if self.current_tick > *duration {
                    self.marked_for_removal = true;
                    self.current_tick = 0;
                }
            }
        };

        if self.current_tick % self.tick_trigger_interval == 0 && !self.marked_for_removal {
            Some(&self.applicable)
        } else {
            None
        }
    }

    pub fn new(
        tick_duration: TickDuration,
        tick_trigger_interval: TickType,
        applicable: ImmediateStatEvent,
    ) -> TickingStatEvent {
        TickingStatEvent {
            tick_duration,
            current_tick: 0,
            tick_trigger_interval,
            applicable,
            marked_for_removal: false,
        }
    }

    pub fn remove(&mut self) {
        self.marked_for_removal = true;
    }
}

#[derive(Default, Component, Debug)]
pub struct TickingStatEventQueue {
    events: Vec<TickingStatEvent>,
}

impl TickingStatEventQueue {
    pub fn push(&mut self, event: TickingStatEvent) {
        self.events.push(event);
    }
}

pub fn init(mut commands: Commands, stat_query: Query<Entity, With<Stats>>) {
    for entity in stat_query.iter() {
        commands
            .entity(entity)
            .insert(TickingStatEventQueue::default());
    }
}

pub fn resolve_ticking_events(
    mut event_queue_query: Query<&mut TickingStatEventQueue>,
    mut stat_event_queue_query: Query<&mut HealthEventQueue>,
) {
    for mut event_queue in event_queue_query.iter_mut() {
        for event in event_queue.events.iter_mut() {
            if let Some(stat_event) = event.tick() {
                let target_entity = stat_event.target_association.target;
                let mut target_stat_queue = stat_event_queue_query
                    .get_mut(target_entity)
                    .expect(format!("Cannot find Target: {:?}", target_entity).as_str());
                target_stat_queue.events.push(*stat_event);
            };
        }
    }
}

pub fn remove_ticking_events(mut event_query: Query<&mut TickingStatEventQueue>) {
    for mut event_queue in event_query.iter_mut() {
        event_queue.events.retain(|event| !event.marked_for_removal);
    }
}
