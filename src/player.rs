use crate::actor::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_rapier3d::na::Quaternion;

#[derive(Component)]
pub struct PlayerMarker;
#[derive(Component)]
pub struct CameraBaseMarker;
#[derive(Component)]
pub struct PlayerCameraMarker;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub actor: Actor,
    pub player: PlayerMarker,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            actor: Default::default(),
            player: PlayerMarker,
        }
    }
}

#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

pub fn move_player(
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const VELOCITY: f32 = 1.0;
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::S) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::A) {
        direction.z -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::LShift) {
        direction.y -= 1.0;
    }

    if direction.length() > 0.001 {
        direction = direction.normalize();
        let mut transform = query.get_single_mut().unwrap();
        transform.translation += direction * VELOCITY * time.delta_seconds();
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tex_handle = asset_server.load("PNG/Green/texture_04.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(tex_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    });

    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    };
    let player_bundle = PlayerBundle {
        actor: Actor { pbr, ..default() },
        ..default()
    };

    let camera_transform = Transform::from_xyz(0.0, 0.0, 4.0);
    let orbit_radius = camera_transform.translation.length();
    // Camera
    let camera = (
        Camera3dBundle {
            transform: camera_transform.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCameraMarker,
    );

    let camera_entity = commands
        .spawn((
            camera,
            PanOrbitCamera {
                radius: orbit_radius,
                ..Default::default()
            },
        ))
        .id();
    commands.spawn(player_bundle).add_child(camera_entity);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(pan_orbit_camera);
    }
}
