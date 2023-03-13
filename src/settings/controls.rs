use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SettingsControlsPlugin;

impl Plugin for SettingsControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_movement_controls.in_base_set(StartupSet::PostStartup))
            .add_startup_system(init_action_bar_controls.in_base_set(StartupSet::PostStartup))
            .add_plugin(InputManagerPlugin::<MovementAction>::default())
            .add_plugin(InputManagerPlugin::<ActionBarAction>::default());
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

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum ActionBarAction {
    Button1,
    Button2,
    Button3,
    Button4,
}

fn init_movement_controls(mut commands: Commands) {
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

fn init_action_bar_controls(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<ActionBarAction> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::Key1, ActionBarAction::Button1),
            (KeyCode::Key2, ActionBarAction::Button2),
            (KeyCode::Key3, ActionBarAction::Button3),
            (KeyCode::Key4, ActionBarAction::Button4),
        ]),
    });
}
