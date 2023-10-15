use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ModelAssets {
    pub scene_handles: HashMap<&'static str, Handle<Scene>>,
}
