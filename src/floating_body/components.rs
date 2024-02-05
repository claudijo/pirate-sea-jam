use crate::utils::hash::{hash_f32_number, hash_vec2};
use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct LinearVelocity(pub Vec2);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct YawRotationalSpeed(pub f32);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Position(pub Vec2);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Yaw(pub f32);

impl Yaw {
    pub fn forward(&self) -> Vec2 {
        let (x, y) = self.0.sin_cos();
        Vec2::new(x, y)
    }

    // pub fn starboard(&self) -> Vec2 {
    //     let (x, y) = (self.0 - FRAC_PI_2).sin_cos();
    //     Vec2::new(x, y)
    // }
}

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Controls {
    pub turn_action: i32,
    pub accelerate_action: i32,
}

pub fn checksum_linear_velocity(value: &LinearVelocity) -> u64 {
    hash_vec2(value.0)
}

pub fn checksum_yaw_rotation_speed(value: &YawRotationalSpeed) -> u64 {
    hash_f32_number(value.0)
}

pub fn checksum_position(value: &Position) -> u64 {
    hash_vec2(value.0)
}

pub fn checksum_yaw(value: &Yaw) -> u64 {
    hash_f32_number(value.0)
}
