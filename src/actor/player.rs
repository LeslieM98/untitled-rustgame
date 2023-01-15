use crate::actor::target::PlayerTarget;
use crate::actor::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_egui::systems::InputEvents;
use bevy_mod_picking::{
    InteractablePickingPlugin, PickingCameraBundle, PickingEvent, PickingPlugin,
};
use bevy_rapier3d::na::inf;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(orbit_camera)
            .add_system(chose_target)
            .add_system(deselect_target)
            .add_system(camera_scroll)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin);
    }
}

#[derive(Component)]
pub struct PlayerMarker;
#[derive(Component)]
pub struct PlayerCameraMarker;
#[derive(Component)]
pub struct CameraBaseNodeMarker;

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

fn orbit_camera(
    windows: Res<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<CameraBaseNodeMarker>>,
) {
    if input_mouse.pressed(MouseButton::Left) {
        let mut transform = query.get_single_mut().unwrap();
        let mouse_delta: Vec2 = mouse_motion_events.iter().map(|x| x.delta).sum();
        let window_size = get_primary_window_size(&windows);

        let delta_y = mouse_delta.y / window_size.y * std::f32::consts::PI;
        let pitch = Quat::from_rotation_x(-delta_y);
        transform.rotation = transform.rotation * pitch; // rotate around local x axis

        let delta_x = mouse_delta.x / window_size.x * std::f32::consts::PI * 2.0;
        let yaw = Quat::from_rotation_y(-delta_x);
        transform.rotation = yaw * transform.rotation; // rotate around global y axis (mind the order of operations)
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
        let mut new_value = transform.translation.z + delta;
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
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

pub fn move_player(
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const VELOCITY: f32 = 3.0;
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

    // Camera
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCameraMarker,
    );

    let camera_entity = commands
        .spawn(camera)
        .insert(PickingCameraBundle::default())
        .id();

    let camera_base_entity = commands
        .spawn((
            TransformBundle::default(),
            CameraBaseNodeMarker,
            VisibilityBundle::default(),
        ))
        .add_child(camera_entity)
        .id();

    commands.spawn(player_bundle).add_child(camera_base_entity);
}

fn chose_target(
    mut commands: Commands,
    mut current_target: Query<Entity, With<PlayerTarget>>,
    mut events: EventReader<PickingEvent>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                for selected_target in current_target.iter_mut() {
                    commands.entity(selected_target).remove::<PlayerTarget>();
                }
                commands.entity(*e).insert(PlayerTarget);
            }
            _ => {}
        }
    }
}

fn deselect_target(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    current_target: Query<Entity, With<PlayerTarget>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if !current_target.is_empty() {
            let entity = current_target.get_single().unwrap();
            commands.entity(entity).remove::<PlayerTarget>();
        }
    }
}
