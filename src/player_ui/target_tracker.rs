use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use crate::player_ui::instantiate_health_bar;
use crate::status_event::stats::Stats;
use bevy::prelude::*;

pub struct TargetTrackerUIPlugin;

impl Plugin for TargetTrackerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, draw);
    }
}

#[derive(Component)]
struct TargetTrackerUIMarker;
#[derive(Component)]
struct TargetTrackerUIHealthBarMarker;

fn draw(
    mut commands: Commands,
    stats_query: Query<&Stats>,
    tracker_query: Query<&Target, With<PlayerMarker>>,
    ui_query: Query<Entity, With<TargetTrackerUIMarker>>,
) {
    ui_query.for_each(|ui_element| commands.entity(ui_element).despawn_recursive());

    for tracker in &tracker_query {
        if let Some(target) = tracker.targeted_entity {
            let health_percentage = stats_query
                .get(target)
                .expect("Cannot find target")
                .get_hp_percentage();

            instantiate_health_bar(
                &mut commands,
                health_percentage,
                100.0,
                100.0,
                0.0,
                0.0,
                TargetTrackerUIMarker,
                TargetTrackerUIHealthBarMarker,
                Color::rgb(1.0, 0.3, 0.3),
                Color::rgb(0.3, 0.3, 0.3),
            );
        }
    }
}
