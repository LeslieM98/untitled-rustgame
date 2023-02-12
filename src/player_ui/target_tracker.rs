use bevy::prelude::*;
use stats_and_abilities_system::prelude::Health;

use crate::actor::player::PlayerMarker;
use crate::actor::target::Target;
use crate::player_ui::widgets::HealthBar;

pub struct TargetTrackerUIPlugin;

impl Plugin for TargetTrackerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, draw);
    }
}

#[derive(Component, Clone)]
struct TargetTrackerUIMarker;
#[derive(Component, Clone)]
struct TargetTrackerUIHealthBarMarker;

fn draw(
    mut commands: Commands,
    health_query: Query<&Health>,
    tracker_query: Query<&Target, With<PlayerMarker>>,
    ui_query: Query<Entity, With<TargetTrackerUIMarker>>,
) {
    ui_query.for_each(|ui_element| commands.entity(ui_element).despawn_recursive());

    for tracker in &tracker_query {
        if let Some(target) = tracker.targeted_entity {
            let health = health_query
                .get(target)
                .expect("Cannot find targeted entity");

            HealthBar::new(TargetTrackerUIMarker, TargetTrackerUIHealthBarMarker)
                .with_width(100.0)
                .with_height(100.0)
                .with_pos_left(0.0)
                .with_pos_bottom(0.0)
                .with_background_color(Color::rgb(0.3, 0.3, 0.3))
                .with_health_color(Color::rgb(1.0, 0.3, 0.3))
                .draw(&mut commands, health);
        }
    }
}
