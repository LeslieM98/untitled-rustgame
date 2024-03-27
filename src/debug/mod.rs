use crate::debug::type_registering::TypeRegisterPlugin;
use bevy::prelude::*;

mod type_registering;

#[derive(SystemSet, Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub struct DebugStage;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_startup_stage_after(StartupSet::PostStartup, DebugStage)
            .add_plugins(TypeRegisterPlugin);
    }
}
