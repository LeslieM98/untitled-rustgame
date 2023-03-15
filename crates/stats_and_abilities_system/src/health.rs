use bevy::prelude::Component;

use crate::events::DamageEvent;
use crate::*;

#[derive(Component, PartialEq)]
pub struct Health {
    current: StatUValueType,
    maximum: StatUValueType,
}

impl Default for Health {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl Health {
    pub fn new(maximum: StatUValueType) -> Self {
        Self {
            maximum,
            current: maximum,
        }
    }

    pub fn get_health_percentage(&self) -> StatFloatType {
        self.current as StatFloatType / self.maximum as StatFloatType
    }

    pub fn current(&self) -> StatUValueType {
        self.current
    }

    pub fn maximum(&self) -> StatUValueType {
        self.current
    }

    pub fn update_maximum(&mut self, new_maximum: StatUValueType) {
        self.maximum = new_maximum;
        if self.current > new_maximum {
            self.current = self.maximum;
        }
    }

    pub fn apply_damage(&mut self, event: &DamageEvent) {
        if self.current < event.value {
            self.current = 0;
        } else {
            self.current -= event.value;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}
