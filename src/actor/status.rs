use bevy::prelude::*;
use bevy::utils::HashMap;

type StatType = i32;
type StatFloatType = f32;

#[derive(Component)]
pub struct Stats {
    pub movement_modifier: StatFloatType,
    pub additional_stats: HashMap<&'static str, StatType>,
}

impl Default for Stats {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Stats::CURR_HP, 100);
        map.insert(Stats::MAX_HP, 100);
        Self {
            movement_modifier: 1.0,
            additional_stats: map,
        }
    }
}

impl Stats {
    pub const MAX_HP: &'static str = "max_hp";
    pub const CURR_HP: &'static str = "curr_hp";
    pub const BASE_VELOCITY: StatFloatType = 3.0;

    pub fn get_max_hp(&self) -> StatType {
        *self.additional_stats.get(Stats::MAX_HP).unwrap()
    }

    pub fn get_current_hp(&self) -> StatType {
        *self.additional_stats.get(Stats::CURR_HP).unwrap()
    }

    pub fn set_current_hp(&mut self, val: StatType) {
        *self.additional_stats.get_mut(Self::CURR_HP).unwrap() = val;
    }

    pub fn get_hp_percentage(&self) -> f32 {
        self.get_current_hp() as f32 / self.get_max_hp() as f32
    }

    pub fn get_movement_velocity(&self) -> StatFloatType {
        self.movement_modifier * Stats::BASE_VELOCITY
    }
}
