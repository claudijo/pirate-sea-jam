use crate::floating_body::systems::float;
use crate::physics::components::{
    checksum_acceleration, checksum_damping, checksum_velocity, Acceleration, Damping, Particle,
    Velocity,
};
use crate::physics::systems::{update_position, update_velocity};
use crate::player::systems::apply_inputs;
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod bundles;
pub mod components;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            (update_velocity, update_position).chain().before(float),
        );

        app.rollback_component_with_copy::<Particle>();
        app.rollback_component_with_copy::<Velocity>();
        app.rollback_component_with_copy::<Acceleration>();
        app.rollback_component_with_copy::<Damping>();

        app.checksum_component::<Velocity>(checksum_velocity);
        app.checksum_component::<Acceleration>(checksum_acceleration);
        app.checksum_component::<Damping>(checksum_damping);
    }
}
