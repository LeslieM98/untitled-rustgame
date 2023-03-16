#![allow(dead_code)]

use crate::events::{resolve_queue, DamageEvent};
use crate::queue::EventQueue;
use bevy::prelude::*;

pub mod events;
pub mod health;
pub mod queue;
pub mod stats;

pub type StatValueType = i32;
pub type StatUValueType = u32;
pub type StatFloatType = f32;
pub type StatIdentifier = String;

#[derive(Default)]
pub struct StatPlugin;

impl Plugin for StatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(event_dispatch::<DamageEvent>)
            .add_event::<DispatchableEvent<DamageEvent>>()
            .add_system(resolve_queue.after(event_dispatch::<DamageEvent>));
    }
}

pub struct DispatchableEvent<T> {
    pub to: Entity,
    pub event: T,
}

impl<T> DispatchableEvent<T> {
    pub fn new(to: Entity, event: T) -> Self {
        Self { to, event }
    }
}

fn event_dispatch<T>(
    mut events: EventReader<DispatchableEvent<T>>,
    mut entity_queues: Query<&mut EventQueue<T>>,
) where
    T: Send + Sync + 'static + Clone,
{
    for event in events.iter() {
        match entity_queues.get_mut(event.to) {
            Ok(mut queue) => queue.content_mut().push(event.event.clone()),
            Err(e) => warn!("{}", e),
        }
    }
}

pub mod prelude {
    pub use crate::health::*;
    pub use crate::stats::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::health::Health;

    fn spawn_entity(app: &mut App) -> Entity {
        app.world
            .spawn((Health::default(), EventQueue::<DamageEvent>::default()))
            .id()
    }

    fn insert_event(
        mut already_inserted: Local<bool>,
        mut events: EventWriter<DispatchableEvent<DamageEvent>>,
    ) {
        if !*already_inserted {
            let event = DispatchableEvent::new(Entity::from_raw(0), DamageEvent { value: 100 });
            events.send(event);
            *already_inserted = true;
        }
    }

    #[test]
    fn correct_event_dispatching() {
        let mut app = App::new();
        app.add_event::<DispatchableEvent<DamageEvent>>()
            .add_system(insert_event);
        app.update();
        let entity1 = spawn_entity(&mut app);
        let entity2 = spawn_entity(&mut app);

        let health1 = app
            .world
            .get::<Health>(entity1)
            .expect("Cannot get this entity");

        let health2 = app
            .world
            .get::<Health>(entity2)
            .expect("Cannot get this entity");

        assert_eq!(health1.current(), 1000);
        assert_eq!(health2.current(), 1000);

        app.add_plugin(StatPlugin::default());
        app.update();

        let health1 = app
            .world
            .get::<Health>(entity1)
            .expect("Cannot get this entity");

        let health2 = app
            .world
            .get::<Health>(entity2)
            .expect("Cannot get this entity");

        assert_eq!(health1.current(), 900);
        assert_eq!(health2.current(), 1000);
    }
}
