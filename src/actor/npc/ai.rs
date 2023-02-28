use crate::actor::npc::NPCMarker;
use crate::actor::player::PlayerMarker;
use bevy::prelude::*;
use stats_and_abilities_system::prelude::StatBlock;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, init)
            .add_system(work_follow_main_task_rotation)
            .add_system(work_move_to_sub_task)
            .add_system(follow_main_task_movement_sub_task_planning);
    }
}

pub enum Task {
    Idle,
    Follow(Entity),
    MoveTo(Vec3),
}
impl Default for Task {
    fn default() -> Self {
        Task::Idle
    }
}

pub enum SubTask {
    MoveTo(Vec3),
    KeepDistance(Vec3, f32),
}

#[derive(Default, Component)]
pub struct AIMovementTaskQueue {
    pub main_task: Task,
    pub sub_task_queue: Vec<SubTask>,
}

fn work_follow_main_task_rotation(
    npc_query: Query<(Entity, &mut AIMovementTaskQueue)>,
    mut transforms: Query<&mut Transform>,
) {
    const EULER: EulerRot = EulerRot::YXZ;
    for (npc, task_queue) in npc_query.iter() {
        if let Task::Follow(target) = task_queue.main_task {
            let target_translation = transforms
                .get(target)
                .expect("Could not find target entity")
                .translation;
            let mut npc_transform = transforms.get_mut(npc).expect("Cannot find NPC");

            let (y_rot, _, _) = npc_transform
                .looking_at(target_translation, npc_transform.up())
                .rotation
                .to_euler(EULER);

            npc_transform.rotation = Quat::from_euler(EULER, y_rot, 0.0, 0.0);
        }
    }
}

fn follow_main_task_movement_sub_task_planning(
    mut npc_query: Query<&mut AIMovementTaskQueue>,
    transform_query: Query<&Transform>,
) {
    for mut task_queue in npc_query.iter_mut() {
        if let Task::Follow(target) = task_queue.main_task {
            let target_translation = transform_query
                .get(target)
                .expect("Cannot find target actor")
                .translation;

            task_queue.sub_task_queue = vec![SubTask::KeepDistance(target_translation, 2.0)];
        }
    }
}

fn work_move_to_sub_task(
    npc_query: Query<(Entity, &StatBlock, &AIMovementTaskQueue)>,
    mut translation_query: Query<&mut Transform>,
    time: Res<Time>,
) {
    for (npc, stats, task_queue) in npc_query.iter() {
        let first_task = task_queue.sub_task_queue.first();
        if let Some(SubTask::KeepDistance(target_translation, distance)) = first_task {
            let mut npc_transform = translation_query.get_mut(npc).expect("Could not find NPC");
            if target_translation.distance(npc_transform.translation) > *distance {
                let translation_delta = npc_transform.forward() * 3.0 * time.delta_seconds();
                npc_transform.translation += translation_delta;
            }
        }
    }
}

fn init(
    mut commands: Commands,
    npc_queue: Query<Entity, With<NPCMarker>>,
    player_queue: Query<Entity, With<PlayerMarker>>,
) {
    if let Some(player) = player_queue.iter().last() {
        for npc in npc_queue.iter() {
            commands.entity(npc).insert(AIMovementTaskQueue {
                main_task: Task::Follow(player),
                ..default()
            });
        }
    } else {
        warn!("Cannot find player");
    }
}
