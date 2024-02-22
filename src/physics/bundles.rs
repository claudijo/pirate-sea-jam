use crate::physics::components::{Damping, ExternalForce, Mass, Particle, Velocity};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub particle: Particle,
    pub velocity: Velocity,
    pub external_force: ExternalForce,
    pub damping: Damping,
    pub mass: Mass,
}
