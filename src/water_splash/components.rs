use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct WaterSplasher {
    // The maximum submersion depth of the object at which point it generates maximum amount of splash.
    pub max_depth: f32,
}
