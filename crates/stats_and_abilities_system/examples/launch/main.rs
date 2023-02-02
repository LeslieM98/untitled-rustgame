use bevy::{log::LogPlugin, prelude::*, time::TimePlugin};
use stats_and_abilities_system::stats::StatBlock;
fn main() {
    App::new()
        .add_plugin(LogPlugin::default())
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(stats_and_abilities_system::StatAbilityPlugin::new(64))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(StatBlock::default());
    commands.spawn(StatBlock::default());
    commands.spawn(StatBlock::default());
    commands.spawn(StatBlock::default());
    commands.spawn(StatBlock::default());
    commands.spawn(StatBlock::default());
}
