use crate::components::ship::{Ship, ShipBooster, ShipFlag, ShipHelm, ShipSail, ShipTurnRate};
use crate::components::shooting_target::ShootingTarget;
use crate::components::wind::Wind;
use crate::utils::number::scale_into_range;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::{PI, TAU};

pub fn push_ship(
    mut ships: Query<(&mut ExternalImpulse, &Transform, &ShipBooster, &Ship)>,
    winds: Query<&Wind>,
) {
    for wind in &winds {
        for (mut external_impulse, transform, booster, ship) in &mut ships {
            let mut ship_forward = transform.local_z();

            ship_forward.y = 0.;
            ship_forward = ship_forward.normalize();

            let wind_speed = wind.direction.length();
            let wind_alignment = wind.direction.dot(ship_forward);
            let wind_factor = scale_into_range(wind_alignment, -wind_speed, wind_speed, 0.6, 1.);

            let boost_factor = if booster.active {
                ship.booster_power
            } else {
                1.
            };

            external_impulse.impulse += ship_forward * ship.speed * wind_factor * boost_factor;
        }
    }
}

pub fn turn_ship(
    mut ships: Query<(
        &mut ExternalImpulse,
        &Transform,
        &Velocity,
        &Ship,
        &ShipTurnRate,
    )>,
) {
    for (mut external_impulse, transform, velocity, ship, rate_of_turn) in &mut ships {
        let mut torque_impulse = Vec3::ZERO;

        let ship_speed = velocity.linvel.xz().length();

        let roll_factor = velocity.angvel.y * ship_speed / ship.stability;
        let yaw_factor =
            -rate_of_turn.value * velocity.linvel.xz().length().sqrt() * ship.maneuverability;

        torque_impulse += transform.local_y() * yaw_factor;
        torque_impulse += transform.local_z() * roll_factor;

        // Pass the vector3 of the axis around which you want to rotate
        external_impulse.torque_impulse += torque_impulse;
    }
}

pub fn rotate_helm(
    rate_of_turns: Query<&ShipTurnRate>,
    mut helms: Query<&mut Transform, With<ShipHelm>>,
) {
    for rate_of_turn in &rate_of_turns {
        for mut transform in &mut helms {
            transform.rotation = Quat::from_rotation_z(rate_of_turn.value * TAU);
        }
    }
}

pub fn flutter_pennant(
    mut pennants: Query<(&mut Transform, &ShipFlag)>,
    rigs: Query<
        (&Transform, Option<&Velocity>),
        (Or<(With<Ship>, With<ShootingTarget>)>, Without<ShipFlag>),
    >,
    winds: Query<&Wind>,
    time: Res<Time>,
) {
    const TIME_SCALE: f32 = 20.;
    let elapsed_time = time.elapsed().as_secs_f32();

    for wind in &winds {
        for (mut pennant_transform, pennant) in &mut pennants {
            if let Some(rig_entity) = pennant.rig {
                if let Ok((rig_transform, rig_velocity)) = rigs.get(rig_entity) {
                    let mut speed_factor = 2_f32.to_radians();
                    if let Some(velocity) = rig_velocity {
                        speed_factor = velocity.linvel.xz().length().to_radians();
                    }
                    let flutter = speed_factor * (elapsed_time * TIME_SCALE).sin();
                    let rig_forward = rig_transform.local_z();
                    let angle = wind.direction.xz().angle_between(rig_forward.xz()) + PI;
                    pennant_transform.rotation = Quat::from_rotation_y(angle + flutter);
                }
            }
        }
    }
}

pub fn flutter_sails(
    mut sails: Query<&mut Transform, With<ShipSail>>,
    ships: Query<&Transform, (With<Ship>, Without<ShipSail>)>,
    winds: Query<&Wind>,
    time: Res<Time>,
) {
    const TIME_SCALE: f32 = 10.;

    let elapsed_time = time.elapsed().as_secs_f32();

    for wind in &winds {
        for ship_transform in &ships {
            for mut sail_transform in &mut sails {
                let ship_forward = ship_transform.local_z();
                let wind_alignment = wind.direction.dot(ship_forward);
                let flutter_factor = wind_alignment - wind.direction.length();

                let flutter = flutter_factor.to_radians() * (elapsed_time * TIME_SCALE).sin();

                sail_transform.rotation = Quat::from_rotation_y(flutter);
            }
        }
    }
}
