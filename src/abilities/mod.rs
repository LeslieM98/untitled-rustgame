use crate::status_event::Stats::*;

pub fn aimed_shot(_source: &Stats, _target: &Stats) -> Stats {
    let mut modification = Stats::empty();
    modification.set_current_hp(-10);
    modification
}

pub fn poison_arrow_tick(_source: &Stats, _target: &Stats) -> Stats {
    let mut modification = Stats::empty();
    modification.set_current_hp(-5);
    modification
}
