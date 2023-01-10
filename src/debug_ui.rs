use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct DebugUI;

impl Plugin for DebugUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(ui_example);
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}
