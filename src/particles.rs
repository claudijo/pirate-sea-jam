pub mod components;
mod systems;

use crate::particles::systems::emit_particles;
use bevy::prelude::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (emit_particles,));
    }
}
