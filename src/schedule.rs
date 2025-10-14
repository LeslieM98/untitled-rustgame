use bevy::app::{App, MainScheduleOrder, PostStartup};
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::Plugin;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PlayerSpawn;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PlayerInit;

pub struct CustomSchedulePlugin;

impl Plugin for CustomSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(PlayerInit);
        app.init_schedule(PlayerSpawn);

        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
        order.insert_startup_after(PostStartup, PlayerSpawn);
        order.insert_startup_after(PlayerSpawn, PlayerInit);
    }
}