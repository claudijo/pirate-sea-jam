use crate::floating_body::systems::float;
use crate::physics::components::{
    checksum_acceleration, checksum_damping, checksum_mass, checksum_velocity, Acceleration,
    Damping, Mass, Particle, Velocity,
};
use crate::physics::resources::Gravity;
use crate::physics::systems::{integrate, update_position, update_velocity};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod bundles;
pub mod components;
mod resources;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity::default());

        app.add_systems(
            GgrsSchedule,
            // (update_velocity, update_position).chain().before(float),
            (integrate).chain().before(float),
        );

        app.rollback_component_with_copy::<Particle>();
        app.rollback_component_with_copy::<Velocity>();
        app.rollback_component_with_copy::<Acceleration>();
        app.rollback_component_with_copy::<Damping>();
        app.rollback_component_with_copy::<Mass>();

        app.checksum_component::<Velocity>(checksum_velocity);
        app.checksum_component::<Acceleration>(checksum_acceleration);
        app.checksum_component::<Damping>(checksum_damping);
        app.checksum_component::<Mass>(checksum_mass);
    }
}
