use crate::ocean::resources::Wave;
use crate::physics::components::{
    AngularDamping, AngularVelocity, Buoy, ExternalForce, ExternalTorque, Inertia, LinearDamping,
    LinearVelocity, Mass,
};
use crate::physics::resources::{Gravity, LiquidDensity};
use bevy::prelude::*;
use bevy_ggrs::Rollback;

pub fn update_angular_velocity(
    mut physics_query: Query<
        (
            &GlobalTransform,
            &Inertia,
            &AngularDamping,
            &ExternalTorque,
            &mut AngularVelocity,
        ),
        With<Rollback>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (global_transform, inertia, angular_damping, external_torque, mut angular_velocity) in
        &mut physics_query
    {
        if inertia.0 == Mat3::ZERO {
            continue;
        }

        let world_coordinates_inertia = inertia.0 * Mat3::from(global_transform.affine().matrix3);
        let angular_acceleration = world_coordinates_inertia.inverse() * external_torque.0;

        // // Update linear velocity from the acceleration.
        angular_velocity.0 += angular_acceleration * delta_time;

        // Impose drag.
        angular_velocity.0 *= angular_damping.0.powf(delta_time);
    }
}

pub fn update_orientation(
    mut physics_query: Query<(&Inertia, &AngularVelocity, &mut Transform), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (inertia, angular_velocity, mut transform) in &mut physics_query {
        if inertia.0 == Mat3::ZERO {
            continue;
        }

        // Update angular position
        transform.rotation *= Quat::from_scaled_axis(angular_velocity.0 * delta_time);
    }
}

pub fn update_linear_velocity(
    gravity: Res<Gravity>,
    mut physics_query: Query<
        (&Mass, &LinearDamping, &ExternalForce, &mut LinearVelocity),
        With<Rollback>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mass, linear_damping, external_force, mut velocity) in &mut physics_query {
        if mass.0 <= 0. {
            continue;
        }

        // Work out the acceleration from the force.
        let mut linear_acceleration = external_force.0 / mass.0;
        linear_acceleration += gravity.0;

        // Update linear velocity from the acceleration.
        velocity.0 += linear_acceleration * delta_time;

        // Impose drag.
        velocity.0 *= linear_damping.0.powf(delta_time);
    }
}

pub fn update_position(
    mut physics_query: Query<(&Mass, &LinearVelocity, &mut Transform), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mass, velocity, mut transform) in &mut physics_query {
        if mass.0 <= 0. {
            continue;
        }

        // Update linear position
        transform.translation += velocity.0 * delta_time;
    }
}

pub fn update_buoyant_force(
    buoy_query: Query<(Entity, &Buoy, &GlobalTransform, &Transform)>,
    mut floating_body_query: Query<&mut ExternalForce>,
    parent_query: Query<&Parent>,
    liquid_density: Res<LiquidDensity>,
) {
    for (entity, buoy, global_transform, transform) in &buoy_query {
        let submerged_proportion =
            (global_transform.translation().y - buoy.water_height - buoy.max_depth)
                / (-2. * buoy.max_depth);

        let force_magnitude = match submerged_proportion {
            s if s <= 0. => 0.,
            s if s >= 1. => buoy.volume * liquid_density.0,
            _ => submerged_proportion * buoy.volume * liquid_density.0,
        };

        for ancestor in parent_query.iter_ancestors(entity) {
            if let Ok(mut external_force) = floating_body_query.get_mut(ancestor) {
                external_force.0 = Vec3::Y * force_magnitude;
            }
        }
    }
}
