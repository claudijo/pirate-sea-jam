use crate::components::ship::{Helm, Ship};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn push_ship(mut ships: Query<(&mut ExternalImpulse, &Transform, &Ship)>) {
    for (mut external_impulse, transform, ship) in &mut ships {
        let mut direction = transform.local_z(); // Forward
        direction.y = 0.;
        direction = direction.normalize();
        external_impulse.impulse = direction * ship.speed_factor;
    }
}

pub fn turn_ship(mut ships: Query<(&mut ExternalForce, &Transform, &Velocity, &Ship)>) {
    let mut torque = Vec3::ZERO;

    for (mut external_force, transform, velocity, ship) in &mut ships {
        torque += transform.local_y().normalize()
            * -ship.turn_rate
            * velocity.linvel.xz().length()
            * ship.maneuverability;

        // Pass the vector3 of the axis around which you want to rotate
        external_force.torque = torque;
    }
}

pub fn rotate_helm(ships: Query<&Ship>, mut helms: Query<&mut Transform, With<Helm>>) {
    for ship in &ships {
        for mut transform in &mut helms {
            transform.rotation = Quat::from_rotation_z(ship.turn_rate.to_radians() * 25.);
        }
    }
}
