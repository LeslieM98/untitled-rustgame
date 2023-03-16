use bevy::prelude::Component;

use crate::events::DamageEvent;
use crate::*;

#[derive(Component, PartialEq, Debug)]
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
        self.maximum
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operation() {
        let mut subject = Health::default();
        assert_eq!(subject.current(), 1000);
        assert_eq!(subject.maximum(), 1000);
        assert_eq!(subject.get_health_percentage(), 1.0);

        subject.apply_damage(&DamageEvent { value: 500 });

        assert_eq!(subject.current(), 500);
        assert_eq!(subject.maximum(), 1000);
        assert_eq!(subject.get_health_percentage(), 0.5);

        subject.update_maximum(800);

        assert_eq!(subject.current(), 500);
        assert_eq!(subject.maximum(), 800);
        assert_eq!(subject.get_health_percentage(), 0.625);

        subject.update_maximum(400);

        assert_eq!(subject.current(), 400);
        assert_eq!(subject.maximum(), 400);
        assert_eq!(subject.get_health_percentage(), 1.0);
    }
}
