use bevy::prelude::*;
use bevy::utils::HashMap;

type StatType = i32;
type StatFloatType = f32;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DamageType {
    Physical,
    Poison,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Damage {
    damage_type: DamageType,
    amount: StatType,
}

impl Damage {
    pub fn new(damage_type: DamageType, amount: StatType) -> Self {
        Self {
            damage_type,
            amount,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Stats {
    pub additional_stats: HashMap<&'static str, StatType>,
}

impl Default for Stats {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Self::CURR_HP, 100);
        map.insert(Self::MAX_HP, 100);
        map.insert(Self::MOVEMENT_SPEED_MODIFIER, 0);
        map.insert(Self::BASE_GLOBAL_COOLDOWN, 1000);
        Self {
            additional_stats: map,
        }
    }
}

impl Stats {
    pub const MAX_HP: &'static str = "max_hp";
    pub const CURR_HP: &'static str = "curr_hp";
    pub const BASE_GLOBAL_COOLDOWN: &'static str = "base_global_cooldown";
    pub const MOVEMENT_SPEED_MODIFIER: &'static str = "movement_speed";

    pub const BASE_MOVEMENT_SPEED: StatType = 1000;
    pub const BASE_VELOCITY: StatFloatType = 3.0;

    pub fn empty() -> Stats {
        Stats {
            additional_stats: Default::default(),
        }
    }

    pub fn get_stat(&self, key: &'static str) -> StatType {
        *self
            .additional_stats
            .get(key)
            .unwrap_or_else(|| panic!("Error getting stat '{}'", key))
    }

    pub fn set_stat(&mut self, key: &'static str, value: StatType) -> Option<StatType> {
        self.additional_stats.insert(key, value)
    }

    pub fn get_max_hp(&self) -> StatType {
        self.get_stat(Self::MAX_HP)
    }

    pub fn get_current_hp(&self) -> StatType {
        self.get_stat(Self::CURR_HP)
    }

    pub fn get_hp_percentage(&self) -> f32 {
        self.get_current_hp() as f32 / self.get_max_hp() as f32
    }

    pub fn get_movement_velocity(&self) -> StatFloatType {
        (self.get_stat(Self::MOVEMENT_SPEED_MODIFIER) + Self::BASE_MOVEMENT_SPEED) as StatFloatType
            / Self::BASE_MOVEMENT_SPEED as StatFloatType
            * Self::BASE_VELOCITY
    }

    /// Applies the damage and returns the over damage.
    pub fn apply_damage(&mut self, damage: &Damage) -> Option<Damage> {
        let curr_hp = self.get_stat(Self::CURR_HP);
        if damage.amount > curr_hp {
            info!("A Changed HP: {:?}", curr_hp);
            let over_damage = Damage::new(damage.damage_type, damage.amount - curr_hp);
            self.set_stat(Self::CURR_HP, 0);
            Some(over_damage)
        } else {
            self.set_stat(Self::CURR_HP, curr_hp - damage.amount);
            info!("B Changed HP: {:?}", self.get_current_hp());
            None
        }
    }
}
