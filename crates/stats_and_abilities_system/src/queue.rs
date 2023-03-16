use bevy::prelude::Component;

#[derive(Component)]
pub struct EventQueue<T> {
    queue: Vec<T>,
}

impl<T> EventQueue<T> {
    pub fn content(&self) -> &Vec<T> {
        &self.queue
    }

    pub fn content_mut(&mut self) -> &mut Vec<T> {
        &mut self.queue
    }
}

impl<T> Default for EventQueue<T> {
    fn default() -> Self {
        EventQueue { queue: Vec::new() }
    }
}
