use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ParticleEmitter {
    pub rate: Timer,
    pub amount_per_burst: usize,
    pub position_variance: f32,
    pub particle_lifetime: f32,
    pub particle_scale_variance: f32,
    pub particle_size: f32,
    pub particle_velocity: Vec3,
    pub particle_color: Color,
    pub particle_alpha_mode: AlphaMode,
}
