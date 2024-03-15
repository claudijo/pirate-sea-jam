use crate::utils::hash::{hash_f32_number, hash_quat, hash_vec3};
use bevy::prelude::shape::Cube;
use bevy::prelude::*;
use bevy::utils::FixedState;
use std::hash::{BuildHasher, Hash, Hasher};

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct LinearVelocity(pub Vec3);

// Vector representing the rotation axis multiplied by the rotation angular speed in rad/s
#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct AngularVelocity(pub Vec3);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct ExternalForce(pub Vec3);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct ExternalTorque(pub Vec3);

// The inertia tensor, unlike the other variables that
// define a rigid body, is given in body space.
#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct Inertia(pub Mat3);

impl Inertia {
    fn non_uniformly_scaled_regular_shape(x: f32, y: f32, z: f32, mass_factor: f32) -> Self {
        if mass_factor <= 0. || x <= 0. || y <= 0. || z <= 0. {
            return Self(Mat3::ZERO);
        }

        let x_squared = x.powi(2);
        let y_squared = y.powi(2);
        let z_squared = z.powi(2);

        Self(Mat3::from_diagonal(Vec3::new(
            mass_factor * (y_squared + z_squared),
            mass_factor * (x_squared + z_squared),
            mass_factor * (x_squared + y_squared),
        )))
    }
    pub fn cuboid(x_length: f32, y_length: f32, z_length: f32, mass: f32) -> Self {
        Self::non_uniformly_scaled_regular_shape(x_length, y_length, z_length, mass / 12.)
    }

    pub fn ellipsoid(x_radius: f32, y_radius: f32, z_radius: f32, mass: f32) -> Self {
        Self::non_uniformly_scaled_regular_shape(x_radius, y_radius, z_radius, mass / 5.)
    }
}

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct SpringStiffness(pub f32);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct SpringDamping(pub f32);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct BendingSpringRestOrientation(pub Quat);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct BendingSpringOrientation(pub Quat);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct TorqueImpulse(pub Vec3);

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct AngularDamping(pub f32);

impl Default for AngularDamping {
    fn default() -> Self {
        AngularDamping(0.999)
    }
}

//  A value of 0.999 might be perfect for damping (pp 50)
#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct LinearDamping(pub f32);

impl Default for LinearDamping {
    fn default() -> Self {
        LinearDamping(0.999)
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

pub fn checksum_linear_velocity(value: &LinearVelocity) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_angular_velocity(value: &AngularVelocity) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_external_force(value: &ExternalForce) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_torque_impulse(value: &TorqueImpulse) -> u64 {
    hash_vec3(value.0)
}

pub fn checksum_spring_stiffness(value: &SpringStiffness) -> u64 {
    hash_f32_number(value.0)
}

pub fn checksum_spring_damping(value: &SpringDamping) -> u64 {
    hash_f32_number(value.0)
}

pub fn checksum_bending_spring_rest_orientation(value: &BendingSpringRestOrientation) -> u64 {
    hash_quat(value.0)
}

pub fn checksum_bending_spring_orientation(value: &BendingSpringOrientation) -> u64 {
    hash_quat(value.0)
}

pub fn checksum_damping(value: &LinearDamping) -> u64 {
    hash_f32_number(value.0)
}

pub fn checksum_mass(value: &Mass) -> u64 {
    hash_f32_number(value.0)
}
