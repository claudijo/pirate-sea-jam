use crate::physics::components::{Acceleration, Damping, Particle, Velocity};
use bevy::prelude::*;

pub fn update_velocity(
    mut physics_query: Query<(&Acceleration, &Damping, &mut Velocity), With<Particle>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (acceleration, damping, mut velocity) in &mut physics_query {
        velocity.0 = velocity.0 * damping.0.powf(delta_time) + acceleration.0 * delta_time;
    }
}

pub fn update_position(
    mut physics_query: Query<(&Velocity, &mut Transform), With<Particle>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (linear_velocity, mut transform) in &mut physics_query {
        transform.translation += linear_velocity.0 * delta_time;
    }
}
