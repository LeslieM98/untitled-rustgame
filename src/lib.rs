use bevy::log::{LogPlugin};
use bevy::prelude::*;
use crate::debug::DebugPlugin;
use crate::player::PlayerPlugin;
use crate::schedule::CustomSchedulePlugin;
use crate::map::MapPlugin;
use bevy::settings::*;
use bevy::window::{ExitCondition, WindowCloseRequested};

pub mod debug;
mod player;
mod schedule;
mod map;


pub fn load_custom_plugins(app: &mut App) {
    app
        .add_plugins(PlayerPlugin)
        .add_plugins(CustomSchedulePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(DebugPlugin);
}

pub fn build_game() -> App{
    let mut app = App::new();
    // ~/.config/sotian.rustgame/settings.toml
    let setting_plugin = SettingsPlugin::new("sotian.rustgame");

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        // We want to intercept the exit so that we can save settings.
        exit_condition: ExitCondition::DontExit,
        primary_window: Some(Window {
            title: "Untitled-Rustgame".into(),
            ..default()
        }),
        ..default()
    }));
    app.add_plugins(setting_plugin);
    app.add_systems(Update, on_window_close);
    load_custom_plugins(&mut app);

    for p in app.get_added_plugins::<LogPlugin>(){
        println!("Debug level: {}", p.level);
    }

    app
}

fn on_window_close(mut close: MessageReader<WindowCloseRequested>, mut commands: Commands) {
    // Save settings immediately, then quit.
    if let Some(_close_event) = close.read().next() {
        commands.queue(SaveSettingsSync::Always);
        commands.write_message(AppExit::Success);
    }
}
