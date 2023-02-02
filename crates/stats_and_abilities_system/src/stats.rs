use std::collections::HashMap;

use bevy::prelude::*;

type StatType = i32;
type StatIdentifier = String;

#[derive(Default)]
pub struct Health {
    current: StatType,
    max: StatType,
}

impl Health {
    pub fn current_mut(&mut self) -> &mut StatType {
        &mut self.current
    }

    pub fn current(&self) -> &StatType {
        &self.current
    }

    pub fn max(&self) -> &StatType {
        &self.max
    }

    pub fn max_mut(&mut self) -> &mut StatType {
        &mut self.max
    }
}

#[derive(Default, Component)]
pub struct Stats {
    health: Health,
    stat_map: HashMap<StatIdentifier, StatType>,
}

impl Stats {
    pub fn insert(&mut self, identifier: StatIdentifier, value: StatType) -> Option<StatType> {
        self.stat_map.insert(identifier, value)
    }

    pub fn get(&self, identifier: &StatIdentifier) -> Option<&StatType> {
        self.stat_map.get(identifier)
    }
}
