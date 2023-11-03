use crate::components::cannon::{Aim, Cannon, CannonBall, Tilt};
use crate::components::ship::Ship;
use crate::components::shooting_target::ShootingTarget;
use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::resources::assets::ModelAssets;
use crate::resources::wave_machine::WaveMachine;
use crate::utils::targeting;
use bevy::prelude::*;
use bevy_rapier3d::geometry::ColliderMassProperties::Density;
use bevy_rapier3d::prelude::*;
use rand::Rng;
pub fn handle_cannon_aim_event(
    shooting_target_query: Query<&Transform, With<ShootingTarget>>,
    ship_query: Query<&Transform, With<Ship>>,
    mut cannon_query: Query<(&mut Aim, &GlobalTransform, &Cannon)>,
    mut aim_cannon_event_reader: EventReader<AimCannonEvent>,
) {
    for event in aim_cannon_event_reader.iter() {
        if let Ok(ship_transform) = ship_query.get(**event) {
            let target_translations = shooting_target_query
                .iter()
                .map(|transform| transform.translation)
                .collect();

            if let Some(closest_target) =
                targeting::find_closest_target(&ship_transform.translation, &target_translations)
            {
                for (mut aim, global_transform, cannon) in &mut cannon_query {
                    if cannon.rig != **event {
                        continue;
                    }

                    let target_direction = *closest_target - global_transform.translation();
                    if global_transform.left().dot(target_direction) > 0. {
                        aim.is_targeting = true;
                    }
                }
            }
        }
    }
}

// Will fire aiming cannons
pub fn handle_cannon_fire_event(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    mut cannon_query: Query<(&GlobalTransform, &mut Aim, &Cannon), Without<Ship>>,
    mut fire_cannon_event_reader: EventReader<FireCannonEvent>,
    mut ship_query: Query<(&Velocity, &mut ExternalImpulse), With<Ship>>,
) {
    for event in fire_cannon_event_reader.iter() {
        let mut rng = rand::thread_rng();

        for (global_transform, mut aim, cannon) in &mut cannon_query {
            if **event != cannon.rig {
                continue;
            }

            if aim.is_targeting {
                aim.is_targeting = false;

                if let Ok((ship_velocity, mut external_impulse)) = ship_query.get_mut(cannon.rig) {
                    // Make ship recoil
                    let recoil_scale = cannon.power * 10.;
                    external_impulse.torque_impulse += global_transform.forward() * recoil_scale;

                    // Spawn cannon ball
                    commands.spawn((
                        SceneBundle {
                            scene: model_assets.scene_handles["cannon_ball.glb"].clone(),
                            transform: Transform::from_translation(global_transform.translation()),
                            ..default()
                        },
                        CannonBall,
                        RigidBody::Dynamic,
                        ExternalImpulse {
                            impulse: global_transform.left()
                                * 20.
                                * cannon.power
                                * rng.gen_range(0.9..1.1),
                            ..default()
                        },
                        Collider::ball(0.3),
                        Density(10.),
                        Velocity {
                            linvel: ship_velocity.linvel,
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

pub fn tilt_cannon(
    mut cannon_query: Query<(&Aim, &mut Tilt, &mut Transform, &Cannon)>,
    mut fire_cannon_event_writer: EventWriter<FireCannonEvent>,
    time: Res<Time>,
) {
    for (aim, mut tilt, mut transform, cannon) in &mut cannon_query {
        let (_, _, z_axis_rotation) = transform.rotation.to_euler(EulerRot::default());

        // Accelerate incline
        if aim.is_targeting
            && tilt.velocity >= 0.
            && z_axis_rotation > -cannon.max_tilt.to_radians()
        {
            tilt.velocity += tilt.acceleration.to_radians() * time.delta_seconds();
            transform.rotation *= Quat::from_rotation_z(-tilt.velocity);
        }

        // Invert tilt velocity at peak angle
        if aim.is_targeting
            && tilt.velocity >= 0.
            && z_axis_rotation <= -cannon.max_tilt.to_radians()
        {
            tilt.velocity *= -1.;
        }

        // Decelerate decline while aiming
        if aim.is_targeting && tilt.velocity < 0. {
            tilt.velocity =
                (tilt.velocity + tilt.acceleration.to_radians() * time.delta_seconds()).min(-0.02);
            transform.rotation *= Quat::from_rotation_z(-tilt.velocity);
        }

        // Stop tilting down and possible force fire cannon
        if tilt.velocity < 0. && z_axis_rotation > 0. {
            tilt.velocity = 0.;
            tilt.stabilize_tilt_timer.reset();

            if aim.is_targeting {
                fire_cannon_event_writer.send(FireCannonEvent(cannon.rig));
            }
        }

        // Invert tilt velocity after firing shot
        if !aim.is_targeting && tilt.velocity >= 0. {
            tilt.velocity *= -1.;
        }

        if !aim.is_targeting && tilt.velocity < 0. {
            tilt.stabilize_tilt_timer.tick(time.delta());
        }

        // Decelerate decline after firing shot after small timeout
        if !aim.is_targeting && tilt.velocity < 0. && tilt.stabilize_tilt_timer.finished() {
            tilt.velocity =
                (tilt.velocity + tilt.acceleration.to_radians() * time.delta_seconds()).min(-0.02);
            transform.rotation *= Quat::from_rotation_z(-tilt.velocity);
        }
    }
}

pub fn despawn_cannon_ball(
    mut commands: Commands,
    cannon_ball_query: Query<(Entity, &GlobalTransform), With<CannonBall>>,
    wave_machine: Res<WaveMachine>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();
    for (entity, global_transform) in &cannon_ball_query {
        let translation = global_transform.translation();
        let water_height = wave_machine.surface_height(translation, elapsed_time);
        if translation.y + 2. < water_height {
            commands.entity(entity).despawn_recursive();
        }
    }
}
