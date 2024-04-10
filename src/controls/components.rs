use crate::utils::hash::{hash_f32_number, hash_vec2};
use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct HelmRotationalSpeed(pub f32);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Controls {
    pub turn_action: i32,
    pub accelerate_action: i32,
}

pub fn checksum_helm_rotation_speed(value: &HelmRotationalSpeed) -> u64 {
    hash_f32_number(value.0)
}
