use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SettingsControlsPlugin;

impl Plugin for SettingsControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(PostStartup, init_movement_controls)
            // .add_systems(PostStartup, init_action_bar_controls)
            .add_plugins(InputManagerPlugin::<MovementAction>::default())
            .add_plugins(InputManagerPlugin::<ActionBarAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MovementAction {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ActionBarAction {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Special1,
    Special2,
    Special3,
    Special4,
}

// fn init_movement_controls(mut commands: Commands) {
//     commands.spawn(InputManagerBundle::<MovementAction> {
//         action_state: ActionState::default(),
//         input_map: InputMap::new([
//             (MovementAction::Forward, KeyCode::KeyW),
//             (MovementAction::Backward, KeyCode::KeyS),
//             (MovementAction::Left, KeyCode::KeyA),
//             (MovementAction::Right, KeyCode::KeyD),
//             (MovementAction::Jump, KeyCode::Space),
//             (MovementAction::Crouch, KeyCode::ShiftLeft),
//         ])
//     });
// }
// 
// fn init_action_bar_controls(mut commands: Commands) {
//     commands.spawn(InputManagerBundle::<ActionBarAction> {
//         action_state: ActionState::default(),
//         input_map: InputMap::new([
//             (ActionBarAction::Button1, KeyCode::Digit1),
//             (ActionBarAction::Button2, KeyCode::Digit2),
//             (ActionBarAction::Button3, KeyCode::Digit3),
//             (ActionBarAction::Button4, KeyCode::Digit4),
//         ]),
//     });
// }
