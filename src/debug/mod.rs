use bevy::prelude::*;


#[derive(SystemSet, Clone, Copy, Hash, Debug, Eq, PartialEq)]
pub struct DebugStage;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {
        // app
            // .add_startup_stage_after(StartupSet::PostStartup, DebugStage)
    }
}
