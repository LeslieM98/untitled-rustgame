use crate::health::Health;
use crate::queue::EventQueue;
use crate::StatUValueType;
use bevy::prelude::{Component, Query};

#[derive(Component, Copy, Clone)]
pub struct DamageEvent {
    pub value: StatUValueType,
}

pub fn resolve_queue(mut damage_queues: Query<(&mut Health, &mut EventQueue<DamageEvent>)>) {
    for (mut health, damage_queue) in &mut damage_queues {
        for event in damage_queue.content() {
            health.apply_damage(event);
        }
    }
}

pub fn reset_queue(mut damage_queues: Query<&mut EventQueue<DamageEvent>>) {
    for mut damage_queue in &mut damage_queues {
        damage_queue.content_mut().clear()
    }
}
