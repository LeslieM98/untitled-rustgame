use crate::actor::target::Target;
use bevy::prelude::*;

pub struct TypeRegisterPlugin;

impl Plugin for TypeRegisterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>();
    }
}
