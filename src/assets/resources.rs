use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ModelAssets {
    pub scene_handles: HashMap<&'static str, Handle<Scene>>,
    pub mesh_handles: HashMap<&'static str, Handle<Mesh>>,
}

#[derive(Resource, Default)]
pub struct FontAssets {
    pub font_handles: HashMap<&'static str, Handle<Font>>,
}
