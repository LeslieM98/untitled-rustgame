use bevy::prelude::*;
use bevy::utils::HashMap;

type StatType = i32;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Stats {
    pub constitution: StatType,
    pub intelligence: StatType,
    pub dexterity: StatType,
    pub strength: StatType,
    pub additional_stats: HashMap<String, StatType>,
}

impl Stats {
    pub fn new(
        constitution: StatType,
        intelligence: StatType,
        dexterity: StatType,
        strength: StatType,
    ) -> Stats {
        Stats {
            constitution,
            intelligence,
            dexterity,
            strength,
            additional_stats: HashMap::default(),
        }
    }
}
