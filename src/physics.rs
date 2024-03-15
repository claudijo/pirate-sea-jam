use crate::floating_body::systems::float;
use crate::physics::components::{
    checksum_angular_velocity, checksum_bending_spring_orientation,
    checksum_bending_spring_rest_orientation, checksum_damping, checksum_external_force,
    checksum_linear_velocity, checksum_mass, checksum_spring_damping, checksum_spring_stiffness,
    checksum_torque_impulse, AngularVelocity, BendingSpringOrientation,
    BendingSpringRestOrientation, ExternalForce, LinearDamping, LinearVelocity, Mass,
    SpringDamping, SpringStiffness, TorqueImpulse,
};
use crate::physics::resources::{Gravity, LiquidDensity};
use crate::physics::systems::{
    update_angular_velocity, update_buoyant_force, update_linear_velocity, update_orientation,
    update_position,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod bundles;
pub mod components;
mod resources;
pub mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity::default());
        app.insert_resource(LiquidDensity::default());

        app.add_systems(
            GgrsSchedule,
            (
                update_buoyant_force,
                update_linear_velocity,
                update_position,
                update_angular_velocity,
                update_orientation,
            )
                .chain()
                .before(float),
        );

        app.rollback_component_with_copy::<LinearVelocity>();
        app.rollback_component_with_copy::<ExternalForce>();
        app.rollback_component_with_copy::<LinearDamping>();
        app.rollback_component_with_copy::<SpringStiffness>();
        app.rollback_component_with_copy::<SpringDamping>();
        app.rollback_component_with_copy::<BendingSpringRestOrientation>();
        app.rollback_component_with_copy::<BendingSpringOrientation>();
        app.rollback_component_with_copy::<TorqueImpulse>();

        app.checksum_component::<LinearVelocity>(checksum_linear_velocity);
        app.checksum_component::<ExternalForce>(checksum_external_force);
        app.checksum_component::<LinearDamping>(checksum_damping);
        app.checksum_component::<Mass>(checksum_mass);
        app.checksum_component::<AngularVelocity>(checksum_angular_velocity);
        app.checksum_component::<TorqueImpulse>(checksum_torque_impulse);
        app.checksum_component::<SpringStiffness>(checksum_spring_stiffness);
        app.checksum_component::<SpringDamping>(checksum_spring_damping);
        app.checksum_component::<BendingSpringRestOrientation>(
            checksum_bending_spring_rest_orientation,
        );
        app.checksum_component::<BendingSpringOrientation>(checksum_bending_spring_orientation);
    }
}
