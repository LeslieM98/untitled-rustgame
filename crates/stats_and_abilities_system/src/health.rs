use bevy::prelude::Component;

use crate::*;

#[derive(Component, PartialEq)]
pub struct Health {
    current: StatValueType,
    maximum: StatValueType,
}

impl Default for Health {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl Health {
    pub fn new(maximum: StatValueType) -> Self {
        Self {
            maximum,
            current: maximum,
        }
    }

    pub fn get_health_percentage(&self) -> StatModifierType {
        todo!()
    }

    pub fn current(&self) -> StatValueType {
        todo!();
    }

    pub fn maximum(&self) -> StatValueType {
        todo!();
    }
}
