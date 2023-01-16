use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SettingsControlsPlugin;

impl Plugin for SettingsControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, init_controls)
            .add_plugin(InputManagerPlugin::<MovementAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MovementAction {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
}

fn init_controls(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<MovementAction> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::W, MovementAction::Forward),
            (KeyCode::S, MovementAction::Backward),
            (KeyCode::A, MovementAction::Left),
            (KeyCode::D, MovementAction::Right),
            (KeyCode::Space, MovementAction::Jump),
            (KeyCode::LShift, MovementAction::Crouch),
        ]),
    });
}
