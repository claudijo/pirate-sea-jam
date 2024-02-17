use crate::physics::components::{Acceleration, Damping, Mass, Particle, Velocity};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub particle: Particle,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub damping: Damping,
    pub mass: Mass,
}
