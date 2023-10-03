// https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier3d/examples/joints_despawn3.rs
use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ShipDespawnEntities {
    pub entities: HashMap<Entity, Vec<Entity>>
}