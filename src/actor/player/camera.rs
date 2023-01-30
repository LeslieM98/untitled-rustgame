use crate::actor::player::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCameraMarker;
#[derive(Component)]
pub struct CameraBaseNodeMarker;

pub fn get_system_set() -> SystemSet {
    SystemSet::new()
        .label("PlayerCameraSystems")
        .with_system(orbit_camera)
        .with_system(camera_scroll)
}

pub fn spawn(commands: &mut Commands) -> Entity {
    // Camera
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCameraMarker,
    );
    let camera_entity = commands
        .spawn(camera)
        .insert(PickingCameraBundle::default())
        .id();

    commands
        .spawn((
            TransformBundle::default(),
            CameraBaseNodeMarker,
            VisibilityBundle::default(),
        ))
        .add_child(camera_entity)
        .id()
}

fn orbit_camera(
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut camera_base: Query<&mut Transform, With<CameraBaseNodeMarker>>,
    mut player: Query<&mut Transform, (With<PlayerMarker>, Without<CameraBaseNodeMarker>)>,
) {
    if input_mouse.pressed(MouseButton::Left) || input_mouse.pressed(MouseButton::Right) {
        let mut camera_transform = camera_base.get_single_mut().unwrap();
        let mut player_transform = player.get_single_mut().unwrap();

        let mouse_delta: Vec2 = mouse_motion_events.iter().map(|x| x.delta).sum();
        let window_size = get_primary_window_size(&windows);

        // up down
        let delta_y = mouse_delta.y / window_size.y * std::f32::consts::PI;
        let pitch = Quat::from_rotation_x(-delta_y);
        let new_up_down_rot = camera_transform.rotation * pitch; // rotate around local x axis
        let up = new_up_down_rot * Vec3::Y;
        let is_upside_down = up.y <= 0.0;
        if !is_upside_down {
            camera_transform.rotation = new_up_down_rot;
        }

        // left right
        let delta_x = mouse_delta.x / window_size.x * std::f32::consts::PI * 2.0;
        let yaw = Quat::from_rotation_y(-delta_x);
        if input_mouse.pressed(MouseButton::Left) {
            // only rotate camera
            let new_left_right_rot = yaw * camera_transform.rotation; // rotate around global y axis (mind the order of operations)
            camera_transform.rotation = new_left_right_rot;
        } else if input_mouse.pressed(MouseButton::Right) {
            // also rotate player only around
            let camera_euler_rot = camera_transform.rotation.to_euler(EulerRot::YXZ).0;
            camera_transform.rotation =
                Quat::from_rotation_y(-camera_euler_rot) * camera_transform.rotation;
            player_transform.rotation =
                Quat::from_rotation_y(camera_euler_rot) * player_transform.rotation;

            let new_left_right_rot = yaw * player_transform.rotation; // rotate around global y axis (mind the order of operations)
            player_transform.rotation = new_left_right_rot; // rotate around global y axis (mind the order of operations)
        }
    }
}

fn camera_scroll(
    mut query: Query<&mut Transform, With<PlayerCameraMarker>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    const MIN_DISTANCE: f32 = 3.0;
    const MAX_DISTANCE: f32 = 20.0;
    if !scroll_events.is_empty() {
        let delta: f32 = scroll_events.iter().map(|event| event.y).sum();
        let mut transform = query.get_single_mut().unwrap();
        let mut new_value = transform.translation.z + -delta;
        new_value = if new_value < MIN_DISTANCE {
            MIN_DISTANCE
        } else if new_value > MAX_DISTANCE {
            MAX_DISTANCE
        } else {
            new_value
        };

        transform.translation.z = new_value;
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width(), window.height())
}
