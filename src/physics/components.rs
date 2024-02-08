use bevy::prelude::*;
use crate::utils::hash::{hash_f32_number, hash_vec2, hash_vec3};

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct PhysicsBody;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct LinearVelocity(pub Vec3);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct LinearAcceleration(pub Vec3);

//  A value of 0.999 might be perfect for damping (pp 50)
#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct LinearDamping(pub f32);

pub fn checksum_linear_velocity(value: &LinearVelocity) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_linear_acceleration(value: &LinearAcceleration) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_linear_damping(value: &LinearDamping) -> u64 {
    hash_f32_number(value.0)
}
