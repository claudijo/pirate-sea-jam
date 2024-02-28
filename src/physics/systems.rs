use crate::physics::components::{Damping, ExternalForce, Mass, Particle, Velocity};
use crate::physics::resources::Gravity;
use bevy::prelude::*;
use bevy_ggrs::Rollback;

// TODO: Split up/rename
pub fn integrate(
    gravity: Res<Gravity>,
    mut particle_query: Query<
        (
            &Mass,
            &Damping,
            &ExternalForce,
            &mut Velocity,
            &mut Transform,
        ),
        With<Rollback>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mass, damping, external_force, mut velocity, mut transform) in &mut particle_query {
        if mass.0 <= 0. {
            continue;
        }

        // Update linear position
        transform.translation += velocity.0 * delta_time;

        // Work out the acceleration from the force.
        let mut resulting_acceleration = external_force.0 / mass.0;
        resulting_acceleration += gravity.0;

        // Update linear velocity from the acceleration.
        velocity.0 += resulting_acceleration * delta_time;

        // Impose drag.
        velocity.0 *= damping.0.powf(delta_time);
    }
}

// Only dependent on the acceleration
pub fn update_velocity(
    gravity: Res<Gravity>,
    mut physics_query: Query<(&Mass, &Damping, &mut Velocity), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mass, damping, mut velocity) in &mut physics_query {
        let external_forces = gravity.0 * mass.0;
        velocity.0 =
            velocity.0 * damping.0.powf(delta_time) + delta_time * external_forces / mass.0;
    }
}

// Position will depend on velocity (acceleration is neglectable)
pub fn update_position(
    mut physics_query: Query<(&Velocity, &mut Transform), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (linear_velocity, mut transform) in &mut physics_query {
        transform.translation += linear_velocity.0 * delta_time;
    }
}
