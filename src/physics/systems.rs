use crate::physics::components::{
    AerofoilArea, AngularDamping, AngularDrag, AngularVelocity, Buoy, ExternalForce,
    ExternalTorque, Inertia, LinearDamping, LinearDrag, LinearVelocity, Mass,
};
use crate::physics::resources::{AirDensity, Gravity, WaterDensity};
use crate::wind::resources::Wind;
use bevy::prelude::*;
use bevy_ggrs::Rollback;

pub fn update_angular_velocity(
    mut physics_query: Query<
        (
            &GlobalTransform,
            &Inertia,
            &AngularDamping,
            &mut ExternalTorque,
            &mut AngularVelocity,
        ),
        With<Rollback>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (global_transform, inertia, angular_damping, mut external_torque, mut angular_velocity) in
        &mut physics_query
    {
        if inertia.0 == Mat3::ZERO {
            continue;
        }

        let world_coordinates_inertia = inertia.0 * Mat3::from(global_transform.affine().matrix3);
        let angular_acceleration = world_coordinates_inertia.inverse() * external_torque.0;

        // Update linear velocity from the acceleration.
        angular_velocity.0 += angular_acceleration * delta_time;

        // Impose drag.(The damping parameter can be flexible and allow it to be used to simulate
        // visible levels of drag)
        angular_velocity.0 *= angular_damping.0.powf(delta_time);

        // Reset torque
        external_torque.0 = Vec3::ZERO;
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
        (
            &Mass,
            &LinearDamping,
            &mut ExternalForce,
            &mut LinearVelocity,
        ),
        With<Rollback>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mass, linear_damping, mut external_force, mut velocity) in &mut physics_query {
        if mass.0 <= 0. {
            continue;
        }

        // Work out the acceleration from the force.
        let mut linear_acceleration = external_force.0 / mass.0;
        linear_acceleration += gravity.0;

        // Update linear velocity from the acceleration.
        velocity.0 += linear_acceleration * delta_time;

        // Impose drag.(The damping parameter can be flexible and allow it to be used to simulate
        // visible levels of drag)
        velocity.0 *= linear_damping.0.powf(delta_time);

        // Reset force
        external_force.0 = Vec3::ZERO;
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

pub fn update_linear_drag_force(
    mut physics_query: Query<(&LinearDrag, &LinearVelocity, &mut ExternalForce)>,
) {
    for (linear_drag, mut linear_velocity, mut external_force) in &mut physics_query {
        let speed = linear_velocity.0.length();
        let drag_coefficient = linear_drag.velocity_drag_coefficient * speed
            + linear_drag.velocity_squared_drag_coefficient * speed.powi(2);
        if linear_velocity.0.length() > 0. {
            let drag_force = linear_velocity.0.normalize() * -drag_coefficient;
            external_force.0 += drag_force;
        }
    }
}

pub fn update_angular_drag_force(
    mut physics_query: Query<(&AngularDrag, &AngularVelocity, &mut ExternalTorque)>,
) {
    for (angular_drag, mut angular_velocity, mut external_torque) in &mut physics_query {
        let speed = angular_velocity.0.length();
        let drag_coefficient = angular_drag.velocity_drag_coefficient * speed
            + angular_drag.velocity_squared_drag_coefficient * speed.powi(2);
        if angular_velocity.0.length() > 0. {
            let drag_force = angular_velocity.0.normalize() * -drag_coefficient;
            external_torque.0 += drag_force;
        }
    }
}

// Assume buoys apply buoyant force to parent
pub fn update_buoyant_force(
    buoy_query: Query<(&Parent, &Buoy, &GlobalTransform, &Transform)>,
    mut floating_body_query: Query<(&GlobalTransform, &mut ExternalForce, &mut ExternalTorque)>,
    water_density: Res<WaterDensity>,
) {
    for (parent, buoy, global_transform, transform) in &buoy_query {
        let submerged_proportion =
            (global_transform.translation().y - buoy.water_height - buoy.max_depth)
                / (-2. * buoy.max_depth);

        let force_magnitude = match submerged_proportion {
            s if s <= 0. => 0.,
            s if s >= 1. => buoy.volume * water_density.0,
            _ => submerged_proportion * buoy.volume * water_density.0,
        };

        if let Ok((parent_global_transform, mut external_force, mut external_torque)) =
            floating_body_query.get_mut(parent.get())
        {
            let force = Vec3::Y * force_magnitude;
            external_torque.0 += (global_transform.translation()
                - parent_global_transform.translation())
            .cross(force);
            external_force.0 += force;
        }
    }
}

pub fn update_aerodynamic_force(
    mut gizmos: Gizmos,
    sail_query: Query<(Entity, &GlobalTransform, &AerofoilArea)>,
    mut vessel_query: Query<(
        &GlobalTransform,
        &LinearVelocity,
        &mut ExternalForce,
        &mut ExternalTorque,
    )>,
    parent_query: Query<&Parent>,
    wind: Res<Wind>,
    air_density: Res<AirDensity>,
) {
    'outer: for (sail_entity, sail_global_transform, aerofoil_area) in &sail_query {
        for parent_entity in parent_query.iter_ancestors(sail_entity) {
            if let Ok((
                vessel_global_transform,
                linear_velocity,
                mut external_force,
                mut external_torque,
            )) = vessel_query.get_mut(parent_entity)
            {
                let apparent_wind = wind.0 - linear_velocity.0;
                let normalized_wind = apparent_wind.normalize();
                let velocity = apparent_wind.length();
                let angle_of_attack = apparent_wind
                    .xz()
                    .angle_between(sail_global_transform.left().xz());
                let dynamic_pressure = 0.5 * air_density.0 * velocity.powi(2);
                let lift_coefficient = (angle_of_attack * 2.).sin();
                let drag_coefficient = 1. - (angle_of_attack * 2.).cos();

                let lift = normalized_wind.cross(Vec3::Y)
                    * dynamic_pressure
                    * aerofoil_area.0
                    * lift_coefficient;
                let drag = normalized_wind * dynamic_pressure * aerofoil_area.0 * drag_coefficient;
                let force = lift + drag;

                gizmos.ray(sail_global_transform.translation(), lift, Color::BLUE);

                gizmos.ray(sail_global_transform.translation(), drag, Color::RED);

                gizmos.ray(sail_global_transform.translation(), force, Color::ORANGE);

                external_torque.0 += (sail_global_transform.translation()
                    - vessel_global_transform.translation())
                .cross(force);
                external_force.0 += force;

                // Make sure each aerodynamic surface affects at most one parent
                continue 'outer;
            }
        }
    }
}
