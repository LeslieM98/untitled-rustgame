#![allow(dead_code)]

use crate::events::{resolve_queue, DamageEvent};
use crate::queue::EventQueue;
use bevy::app::{App, Plugin};
use bevy::log::warn;
use bevy::prelude::*;

pub mod events;
pub mod health;
pub mod queue;
pub mod stats;

pub type StatValueType = i32;
pub type StatUValueType = u32;
pub type StatFloatType = f32;
pub type StatIdentifier = String;

pub struct StatPlugin;

impl Plugin for StatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(event_dispatch)
            .add_system(resolve_queue.after(event_dispatch));
    }
}

pub struct DispatchableEvent<T> {
    pub to: Entity,
    pub event: T,
}

fn event_dispatch(
    mut events: EventReader<DispatchableEvent<DamageEvent>>,
    mut entity_queues: Query<&mut EventQueue<DamageEvent>>,
) {
    for event in events.iter() {
        match entity_queues.get_mut(event.to) {
            Ok(mut queue) => queue.content_mut().push(event.event),
            Err(e) => warn!("{}", e),
        }
    }
}

pub mod prelude {
    pub use crate::health::*;
    pub use crate::stats::*;
}
