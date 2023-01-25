use crate::status_event::Stats::DamageType::Physical;
use crate::status_event::Stats::*;
use bevy::prelude::info;

pub fn aimed_shot(_source: &Stats, _target: &mut Stats) {
    _target.apply_damage(&Damage::new(Physical, 10));
}

pub fn poison_arrow_tick(_source: &Stats, _target: &mut Stats) {
    _target.apply_damage(&Damage::new(Physical, 5));
}
