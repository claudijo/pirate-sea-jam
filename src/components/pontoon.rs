use bevy::prelude::*;

#[derive(Component)]
pub struct CubePontoonSize {
    pub side: f32,
}

#[derive(Component)]
pub struct PontoonForceScale {
    pub buoyant_force_scale: f32,
    pub water_damping_scale: f32,
}
