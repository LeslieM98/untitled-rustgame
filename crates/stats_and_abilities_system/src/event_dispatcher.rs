use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct EventDispatcher {
    events: Vec<i32>,
}

impl EventDispatcher {
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
