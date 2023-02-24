use bevy_ecs::prelude::Component;

use crate::StatValueType;

#[derive(Component)]
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

    pub fn get_current(&self) -> StatValueType {
        self.current
    }

    pub fn get_maximum(&self) -> StatValueType {
        self.maximum
    }

    pub fn get_health_percentage(&self) -> f32 {
        self.current as f32 / self.maximum as f32
    }
    
    pub fn apply_damage(&mut self, value: StatValueType) -> StatValueType {
        let new_health = self.current - value;
        if new_health < 0 {
            self.current = 0;
            -new_health
        } else {
            self.current = new_health;
            0
        }
    }

    pub fn apply_heal(&mut self, value: StatValueType) -> StatValueType {
        let new_health = self.current + value;
        if new_health > self.maximum {
            self.current = self.maximum;
            new_health - self.maximum
        } else {
            self.current = new_health;
            0
        }
    }
}

pub mod events {
    use crate::health::Health;
    use crate::stats::StatBlock;
    use crate::StatValueType;

    type DamageApplicationType = Box<dyn FnOnce(&StatValueType, &mut Health, &StatBlock)>;

    pub struct DamageEvent{
        value: StatValueType,
        application: DamageApplicationType
    }

    impl Default for DamageEvent {
        fn default() -> Self {
            Self{
                value: 0,
                application: Box::new(|value, target_health, _target_stats| {target_health.apply_damage(*value);})
            }
        }
    }

    impl DamageEvent {
        pub fn apply(self, target_health: &mut Health, target_stats: &StatBlock) {
            let application = self.application;
            application(&self.value, target_health, target_stats);
        }

        pub fn with_default_application(value: StatValueType) -> Self{
            Self{
                value, ..Self::default()
            }
        }

        pub fn new(value: StatValueType, application: DamageApplicationType) -> Self {
            Self { value, application }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_damage() {
        let mut subject = Health::new(100);
        let over_damage = subject.apply_damage(99);
        assert_eq!(over_damage, 0);
        assert_eq!(subject.get_current(), 1);

        let over_damage = subject.apply_damage(50);
        assert_eq!(over_damage, 49);
        assert_eq!(subject.get_current(), 0);
    }

    #[test]
    fn apply_heal() {
        let mut subject = Health::new(100);
        subject.apply_damage(50);
        assert_eq!(subject.get_current(), 50);
        let over_heal = subject.apply_heal(49);
        assert_eq!(subject.get_current(), 99);
        assert_eq!(over_heal, 0);
        let over_heal = subject.apply_heal(50);
        assert_eq!(subject.get_current(), subject.get_maximum());
        assert_eq!(over_heal, 49);
    }

    mod event {
        use crate::health::events::DamageEvent;
        use crate::health::Health;
        use crate::stats::StatBlock;

        #[test]
        fn apply_default_damage_event() {
            let event = DamageEvent::default();
            let mut health = Health::new(100);
            let stats = StatBlock::default();

            assert_eq!(health.current, 100);
            event.apply(&mut health, &stats);
            assert_eq!(health.current, 100);

            let event = DamageEvent::with_default_application(20);
            event.apply(&mut health, &stats);
            assert_eq!(health.current, 80);
        }
    }
}
