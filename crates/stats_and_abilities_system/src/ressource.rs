use bevy::prelude::Component;
use std::collections::HashMap;

use crate::{StatIdentifier, StatValueType};

pub struct ActorResource {
    pub identifier: StatIdentifier,
    pub current: StatValueType,
    pub maximum: StatValueType,
}

#[derive(Default, Component)]
pub struct ActorRessourceBundle {
    pub resources: HashMap<StatIdentifier, ActorResource>,
}

impl ActorRessourceBundle {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(resource: ActorResource) -> Self {
        let mut r = Self::empty();
        r.resources.insert(resource.identifier.clone(), resource);
        r
    }

    pub fn get_first(&self) -> Option<&ActorResource> {
        for (_, v) in &self.resources {
            return Some(v);
        }
        return None;
    }

    pub fn get_first_mut(&mut self) -> Option<&mut ActorResource> {
        for (_, v) in &mut self.resources {
            return Some(v);
        }
        return None;
    }
}

impl ActorResource {
    pub fn new(identifier: StatIdentifier, maximum: StatValueType) -> Self {
        Self {
            identifier,
            maximum,
            current: maximum,
        }
    }
    pub fn get_percentage(&self) -> f32 {
        self.current as f32 / self.maximum as f32
    }
}
