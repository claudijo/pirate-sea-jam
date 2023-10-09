// https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier3d/examples/joints_despawn3.rs
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ShipDespawnEntities {
    pub entities: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct ShootingTargetDespawnEntities {
    pub entities: Vec<Entity>,
}
