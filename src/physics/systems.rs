use crate::physics::components::{Damping, ExternalForce, Mass, Velocity};
use crate::physics::resources::Gravity;
use bevy::prelude::*;
use bevy_ggrs::Rollback;

pub fn update_velocity(
    gravity: Res<Gravity>,
    mut particle_query: Query<(&Mass, &Damping, &ExternalForce, &mut Velocity), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mass, damping, external_force, mut velocity) in &mut particle_query {
        if mass.0 <= 0. {
            continue;
        }

        // Work out the acceleration from the force.
        let mut resulting_acceleration = external_force.0 / mass.0;
        resulting_acceleration += gravity.0;

        // Update linear velocity from the acceleration.
        velocity.0 += resulting_acceleration * delta_time;

        // Impose drag.
        velocity.0 *= damping.0.powf(delta_time);
    }
}

pub fn update_position(
    mut particle_query: Query<(&Mass, &mut Velocity, &mut Transform), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mass, mut velocity, mut transform) in &mut particle_query {
        if mass.0 <= 0. {
            continue;
        }

        // Update linear position
        transform.translation += velocity.0 * delta_time;
    }
}
