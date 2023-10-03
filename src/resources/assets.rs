use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ShipAssets {
    pub scene_handles: HashMap<&'static str, Handle<Scene>>,
}
