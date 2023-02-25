use crate::health::events::DamageEvent;
use crate::health::Health;
use crate::stats::StatBlock;
use bevy::prelude::*;
use std::collections::vec_deque::VecDeque;

struct DispatchableEvent<T> {
    pub to: Entity,
    pub from: Entity,
    pub event: T,
}

impl<T> DispatchableEvent<T> {
    pub fn new(to: Entity, from: Entity, event: T) -> Self {
        Self { to, from, event }
    }
}

#[derive(Resource, Default)]
struct EventDispatcher {
    damage: VecDeque<DispatchableEvent<DamageEvent>>,
}

#[derive(Component)]
struct DamageQueue {
    queue: VecDeque<DispatchableEvent<DamageEvent>>,
}

struct EventDispatcherPlugin;

impl Plugin for EventDispatcherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EventDispatcher::default())
            .add_system(dispatch_damage_queue)
            .add_system(resolve_damage_queues);
    }
}

fn dispatch_damage_queue(
    mut event_dispatcher: ResMut<EventDispatcher>,
    mut entity_queues: Query<&mut DamageQueue>,
) {
    while let Some(damage_event) = event_dispatcher.damage.pop_front() {
        match entity_queues.get_mut(damage_event.to) {
            Ok(mut target_queue) => target_queue.queue.push_back(damage_event),
            Err(_) => error!("Cannot find target"),
        }
    }
}

fn resolve_damage_queues(mut damage_queues: Query<(&mut DamageQueue, &StatBlock, &mut Health)>) {
    for (mut damage_queue, target_stats, mut target_health) in &mut damage_queues {
        while let Some(event) = damage_queue.queue.pop_front() {
            event.event.apply(&mut target_health, target_stats)
        }
    }
}
