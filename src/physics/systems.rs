use crate::physics::components::{
    LinearAcceleration, LinearDamping, LinearVelocity, PhysicsBody,
};
use bevy::prelude::*;

pub fn update_velocity(
    mut physics_query: Query<
        (&LinearAcceleration, &LinearDamping, &mut LinearVelocity),
        With<PhysicsBody>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (linear_acceleration, linear_damping, mut linear_velocity) in &mut physics_query {
        linear_velocity.0 =
            linear_velocity.0 * linear_damping.0.powf(delta_time) + linear_acceleration.0 * delta_time;
    }
}

pub fn update_position(
    mut physics_query: Query<(&LinearVelocity, &mut Transform), With<PhysicsBody>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (linear_velocity, mut transform) in &mut physics_query {
        transform.translation += linear_velocity.0 * delta_time;
    }
}
