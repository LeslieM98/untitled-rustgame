use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Target {
    pub targeted_entity: Option<Entity>,
}

#[derive(Component, Default)]
pub struct Targetable;
