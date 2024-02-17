use crate::utils::hash::{hash_f32_number, hash_vec3};
use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Particle;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct Acceleration(pub Vec3);

impl Default for Acceleration {
    fn default() -> Self {
        Acceleration(Vec3::NEG_Y * 15.)
    }
}

//  A value of 0.999 might be perfect for damping (pp 50)
#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct Damping(pub f32);

impl Default for Damping {
    fn default() -> Self {
        Damping(0.999)
    }
}

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1.) // 1kg
    }
}

pub fn checksum_velocity(value: &Velocity) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_acceleration(value: &Acceleration) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_damping(value: &Damping) -> u64 {
    hash_f32_number(value.0)
}

pub fn checksum_mass(value: &Mass) -> u64 {
    hash_f32_number(value.0)
}