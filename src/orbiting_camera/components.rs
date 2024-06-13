use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct OrbitingCamera {
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
}

impl Default for OrbitingCamera {
    fn default() -> Self {
        OrbitingCamera {
            radius: 10.,
            pitch: 30_f32.to_radians(),
            yaw: 0.,
            min_pitch: 10_f32.to_radians(),
            max_pitch: FRAC_PI_2,
        }
    }
}
