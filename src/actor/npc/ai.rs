use crate::actor::npc::NPCMarker;
use crate::actor::player::PlayerMarker;
use crate::actor::status::Stats;
use bevy::prelude::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init)
            .add_system(movement)
            .add_system(debug_loop);
    }
}

pub struct PositionalTask {
    target_translation: Vec3,
}

#[derive(Component, Default)]
pub struct PositionalTaskQueue {
    pub queue: Vec<PositionalTask>,
}

fn movement(mut npc_query: Query<(&Stats, &mut Transform, &PositionalTaskQueue)>, time: Res<Time>) {
    for (stats, mut transform, task_queue) in npc_query.iter_mut() {
        if let Some(task) = task_queue.queue.first() {
            let (up_axis_rotation, _, _) = transform
                .looking_at(task.target_translation, Vec3::Y)
                .rotation
                .to_euler(EulerRot::default());

            transform.rotation = Quat::from_euler(EulerRot::default(), up_axis_rotation, 0.0, 0.0);

            let mut direction = Vec3::ZERO;
            let velocity = stats.get_movement_velocity();
            direction += transform.forward();
            transform.translation += direction * velocity * time.delta_seconds();
        }
    }
}

fn debug_loop(
    player_query: Query<&Transform, With<PlayerMarker>>,
    mut npc_query: Query<&mut PositionalTaskQueue>,
) {
    let player_transform = player_query.get_single().unwrap();
    for mut task_queue in npc_query.iter_mut() {
        let task = PositionalTask {
            target_translation: player_transform.translation,
        };
        task_queue.queue.clear();
        task_queue.queue.push(task);
    }
}

fn init(npc_query: Query<Entity, With<NPCMarker>>, mut commands: Commands) {
    for entity in npc_query.iter() {
        commands
            .entity(entity)
            .insert(PositionalTaskQueue::default());
    }
}
