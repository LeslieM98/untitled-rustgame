use bevy::prelude::*;

type HealthType = u32;

#[derive(Component)]
pub struct BaseHealth {
    pub max_hp: HealthType,
    pub curr_hp: HealthType,
}

impl Default for BaseHealth {
    fn default() -> Self {
        Self {
            max_hp: 100,
            curr_hp: 100,
        }
    }
}

impl BaseHealth {
    pub fn apply_heal(&mut self, amount: HealthType) -> HealthType {
        let new_hp = self.curr_hp + amount;
        let overheal = if new_hp > self.max_hp {
            new_hp - self.max_hp
        } else {
            0
        };
        self.curr_hp = new_hp;
        overheal
    }

    pub fn apply_damage(&mut self, amount: HealthType) -> HealthType {
        if amount > self.curr_hp {
            let overkill = amount - self.curr_hp;
            self.curr_hp = 0;
            overkill
        } else {
            self.curr_hp -= amount;
            0
        }
    }
}