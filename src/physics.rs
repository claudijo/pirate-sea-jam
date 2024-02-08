use crate::physics::systems::{update_position, update_velocity};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};
use crate::floating_body::systems::float;
use crate::physics::components::{checksum_linear_acceleration, checksum_linear_damping, checksum_linear_velocity, LinearAcceleration, LinearDamping, LinearVelocity, PhysicsBody};
use crate::player::systems::apply_inputs;

pub mod components;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(GgrsSchedule, (update_velocity, update_position).chain().before(float));

        app.rollback_component_with_copy::<PhysicsBody>();
        app.rollback_component_with_copy::<LinearVelocity>();
        app.rollback_component_with_copy::<LinearAcceleration>();
        app.rollback_component_with_copy::<LinearDamping>();

        app.checksum_component::<LinearVelocity>(checksum_linear_velocity);
        app.checksum_component::<LinearAcceleration>(checksum_linear_acceleration);
        app.checksum_component::<LinearDamping>(checksum_linear_damping);
    }
}
