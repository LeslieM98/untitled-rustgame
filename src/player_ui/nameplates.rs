use crate::actor::player::camera::PlayerCameraMarker;
use crate::player_ui::instantiate_health_bar;
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
    instantiate_health_bar(
        commands,
        health_percentage,
        100.0,
        20.0,
        position.x,
        position.y,
        NamePlateUIMarker,
        NamePlateUIHealthBarMarker,
        Color::rgb(1.0, 0.3, 0.3),
        Color::rgb(0.3, 0.3, 0.3),
    )
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
