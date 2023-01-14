use crate::player::PlayerMarker;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct DebugUI;

impl Plugin for DebugUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(player_position_ui);
    }
}

fn player_position_ui(
    mut egui_context: ResMut<EguiContext>,
    query: Query<&Transform, With<PlayerMarker>>,
    time: Res<Time>,
) {
    let transform = query.get_single().unwrap();
    egui::Window::new("Player Position").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Player X: {}", transform.translation.x));
        ui.label(format!("Player Y: {}", transform.translation.y));
        ui.label(format!("Player Z: {}", transform.translation.z));
        ui.label(format!("FPS: {}", 60.0 / time.delta_seconds()))
    });
}
