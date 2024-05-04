use crate::physics::components::{
    AngularDamping, AngularVelocity, ExternalForce, ExternalImpulse, ExternalTorque,
    ExternalTorqueImpulse, Inertia, LinearDamping, LinearVelocity, Mass,
};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub linear_velocity: LinearVelocity,
    pub external_force: ExternalForce,
    pub linear_damping: LinearDamping,
    pub external_impulse: ExternalImpulse,
    pub mass: Mass,
}

#[derive(Bundle, Default)]
pub struct SpindleBundle {
    pub angular_velocity: AngularVelocity,
    pub external_torque: ExternalTorque,
    pub angular_damping: AngularDamping,
    pub external_torque_impulse: ExternalTorqueImpulse,
    pub inertia: Inertia,
}
