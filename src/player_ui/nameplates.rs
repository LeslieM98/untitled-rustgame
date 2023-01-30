use crate::actor::player::camera::PlayerCameraMarker;
use crate::status_event::stats::*;
use bevy::app::App;
use bevy::prelude::*;

pub struct NamePlateUIPlugin;

#[derive(Component)]
pub struct NamePlateUIMarker;
#[derive(Component)]
pub struct NamePlateUIHealthBarMarker;

impl Plugin for NamePlateUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, clear_ui)
            .add_system(draw);
    }
}

fn instantiate(commands: &mut Commands, health_percentage: f32, position: &Vec2) {
    let width = 100.0;
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(position.x),
                    bottom: Val::Px(position.y),
                    ..default()
                },
                ..default()
            },
            background_color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..default()
        })
        .insert(NamePlateUIMarker)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(width * health_percentage), Val::Px(20.0)),
                        position_type: PositionType::Relative,
                        position: UiRect {
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                        ..default()
                    },
                    background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                    ..default()
                })
                .insert(NamePlateUIHealthBarMarker);
        });
}

fn clear_ui(mut commands: Commands, ui_query: Query<Entity, With<NamePlateUIMarker>>) {
    for ui in &ui_query {
        commands.entity(ui).despawn_recursive();
    }
}

fn draw(
    mut commands: Commands,
    stat_query: Query<(&GlobalTransform, &Stats)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCameraMarker>>,
) {
    let (camera, camera_transform) = camera_query.get_single().expect("Player camera not found");
    for (actor_transform, actor_stats) in &stat_query {
        if let Some(ui_position) =
            camera.world_to_viewport(camera_transform, actor_transform.translation())
        {
            instantiate(&mut commands, actor_stats.get_hp_percentage(), &ui_position);
        }
    }
}
