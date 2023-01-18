use bevy::prelude::*;

type HealthType = u32;

#[derive(Component)]
pub struct Health {
    base_hp: HealthType,
    curr_hp: HealthType,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            base_hp: 100,
            curr_hp: 100,
        }
    }
}

impl Health {
    pub fn with_current_health(curr_hp: HealthType) -> Self {
        Self {
            curr_hp,
            ..default()
        }
    }
    pub fn apply_heal(&mut self, amount: HealthType) -> HealthType {
        let new_hp = self.curr_hp + amount;
        let overheal = if new_hp > self.base_hp {
            new_hp - self.base_hp
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

    pub fn get_max_hp(&self) -> HealthType {
        return self.base_hp;
    }

    pub fn get_percentage(&self) -> f32 {
        let max_hp = self.get_max_hp();
        if self.curr_hp == 0 {
            0.0
        } else if self.curr_hp == max_hp {
            1.0
        } else {
            self.curr_hp as f32 / max_hp as f32
        }
    }

    pub fn is_dead(&self) -> bool {
        return self.curr_hp == 0;
    }
}
