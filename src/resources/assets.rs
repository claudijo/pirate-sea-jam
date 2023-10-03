use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ShipAssets {
    pub scene_handles: HashMap<&'static str, Handle<Scene>>,
}