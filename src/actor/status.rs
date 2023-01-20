use bevy::prelude::*;
use bevy::utils::HashMap;

type StatType = i32;

#[derive(Component)]
pub struct Stats {
    additional_stats: HashMap<&'static str, StatType>,
}

impl Default for Stats {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Stats::CURR_HP, 100);
        map.insert(Stats::MAX_HP, 100);
        Self {
            additional_stats: map,
        }
    }
}

impl Stats {
    pub const MAX_HP: &'static str = "max_hp";
    pub const CURR_HP: &'static str = "curr_hp";

    pub fn get_max_hp(&self) -> StatType {
        *self.additional_stats.get(Stats::MAX_HP).unwrap()
    }

    pub fn get_current_hp(&self) -> StatType {
        *self.additional_stats.get(Stats::CURR_HP).unwrap()
    }

    pub fn get_hp_percentage(&self) -> f32 {
        self.get_current_hp() as f32 / self.get_max_hp() as f32
    }
}
