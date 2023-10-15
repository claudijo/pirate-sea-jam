use bevy::prelude::*;

#[derive(Component)]
pub struct Pontoon {
    pub radius: f32,
    pub buoyant_force_scale: f32,
    pub water_damping_scale: f32,
}

impl Default for Pontoon {
    fn default() -> Self {
        Self {
            radius: 1.,
            buoyant_force_scale: 0.01,
            water_damping_scale: 0.2,
        }
    }
}
