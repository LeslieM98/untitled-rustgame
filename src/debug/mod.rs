use crate::debug::fps::FPSLabelPlugin;
use crate::debug::type_registering::TypeRegisterPlugin;
use bevy::prelude::*;

mod fps;
mod type_registering;

#[derive(StageLabel)]
pub struct DebugStage;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage_after(
            StartupStage::PostStartup,
            DebugStage,
            SystemStage::single_threaded(),
        )
        .add_plugin(FPSLabelPlugin)
        .add_plugin(TypeRegisterPlugin);
    }
}
