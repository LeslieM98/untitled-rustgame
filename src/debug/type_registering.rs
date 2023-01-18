use crate::actor::health::Health;
use crate::actor::status::Stats;
use bevy::prelude::*;

pub struct TypeRegisterPlugin;

impl Plugin for TypeRegisterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>().register_type::<Stats>();
    }
}
