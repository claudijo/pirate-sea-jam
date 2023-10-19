use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct TouchTrailEntities {
    pub for_touch_id: HashMap<u64, Vec<Entity>>,
}
