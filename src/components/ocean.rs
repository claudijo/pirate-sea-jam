use bevy::prelude::*;

#[derive(Component)]
pub struct OceanTopology {
    pub positions: Vec<[f32; 3]>,
}
