use crate::physics::components::{Damping, Mass, Particle, Velocity};
use crate::physics::resources::Gravity;
use bevy::prelude::*;
use bevy_ggrs::Rollback;

pub fn update_velocity(
    gravity: Res<Gravity>,
    mut physics_query: Query<(&Mass, &Damping, &mut Velocity), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mass, damping, mut velocity) in &mut physics_query {
        let external_forces = gravity.0 * mass.0;
        velocity.0 = velocity.0 * damping.0.powf(delta_time) + delta_time * external_forces / mass.0;
    }
}

pub fn update_position(
    mut physics_query: Query<(&Velocity, &mut Transform), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (linear_velocity, mut transform) in &mut physics_query {
        transform.translation += linear_velocity.0 * delta_time;
    }
}
