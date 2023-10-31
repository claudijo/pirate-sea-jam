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

// Will start aiming cannons facing closes target
pub fn handle_cannon_aim_event(
    shooting_target_query: Query<&Transform, With<ShootingTarget>>,
    ship_query: Query<&Transform, With<Ship>>,
    mut cannon_query: Query<(Entity, &mut Aim, &mut Velocity, &GlobalTransform, &Cannon)>,
    mut event_reader: EventReader<AimCannonEvent>,
) {
    for event in event_reader.iter() {
        if let Ok(ship_transform) = ship_query.get(event.source) {
            let target_translations = shooting_target_query
                .iter()
                .map(|transform| transform.translation)
                .collect();

            if let Some(closest_target) =
                targeting::find_closest_target(&ship_transform.translation, &target_translations)
            {
                for (entity, mut aim, mut velocity, global_transform, cannon) in &mut cannon_query {
                    if cannon.rig != event.source {
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
    mut cannon_query: Query<(Entity, &GlobalTransform,&mut Velocity, &mut Aim, &Cannon), Without<Ship>>,
    mut event_reader: EventReader<FireCannonEvent>,
    mut ship_query: Query<(&Velocity, &mut ExternalImpulse), With<Ship>>,
) {
    for event in event_reader.iter() {
        let mut rng = rand::thread_rng();

        for (entity, global_transform, mut velocity, mut aim, cannon) in &mut cannon_query {
            if event.source != cannon.rig {
                continue;
            }

            if aim.is_targeting {
                aim.is_targeting = false;

                if let Ok((ship_velocity, mut external_impulse)) = ship_query.get_mut(cannon.rig) {
                    // Make ship recoil
                    let recoil_scale = cannon.power * 10.;
                    external_impulse.torque_impulse +=
                        global_transform.forward() * recoil_scale;

                    // Spawn cannon ball
                    commands.spawn((
                        SceneBundle {
                            scene: model_assets.scene_handles["cannon_ball.glb"].clone(),
                            transform: Transform::from_translation(
                                global_transform.translation(),
                            ),
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
    mut cannon_query: Query<(&Aim, &Transform, &mut Velocity, &Cannon)>,
    mut event_writer: EventWriter<FireCannonEvent>,
    time: Res<Time>,
) {
    for (aim, transform, mut velocity, cannon) in &mut cannon_query {
        let (_, _, tilt) = transform.rotation.to_euler(EulerRot::default());

        // Start tilting up
        if aim.is_targeting && velocity.angvel.z == 0. {
            velocity.angvel.z = cannon.tilt_torque * time.delta_seconds();
        }

        // Accelerate tilting up
        if aim.is_targeting && velocity.angvel.z > 0. {
            velocity.angvel.z += cannon.tilt_torque * time.delta_seconds();
        }

        // Start tilting down
        if aim.is_targeting && velocity.angvel.z > 0. && tilt < -45_f32.to_radians() {
            velocity.angvel.z = -cannon.tilt_torque * time.delta_seconds();
        }

        // Accelerate tilting down
        if aim.is_targeting && velocity.angvel.z < 0. {
            velocity.angvel.z += -cannon.tilt_torque * time.delta_seconds();
        }

        // Stop tilting down and force fire cannon
        if aim.is_targeting && velocity.angvel.z < 0. && tilt > 0_f32.to_radians() {
            velocity.angvel.z = 0.;
            event_writer.send(FireCannonEvent { source: cannon.rig });
        }

        // Start tilting down after firing shot
        if !aim.is_targeting && velocity.angvel.z > 0. {
            velocity.angvel.z = -cannon.tilt_torque * time.delta_seconds();
        }

        // Accelerate tilting down after firing shot
        if !aim.is_targeting && velocity.angvel.z < 0. {
            velocity.angvel.z += -cannon.tilt_torque * time.delta_seconds();
        }

        // Stop tilting down after firing shot
        if !aim.is_targeting && velocity.angvel.z < 0. && tilt > 0_f32.to_radians() {
            velocity.angvel.z = 0.;
        }
    }
}

pub fn despawn_cannon_ball(
    mut commands: Commands,
    cannon_balls: Query<(Entity, &GlobalTransform), With<CannonBall>>,
    wave_machine: Res<WaveMachine>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();
    for (entity, global_transform) in &cannon_balls {
        let translation = global_transform.translation();
        let water_height = wave_machine.surface_height(translation, elapsed_time);
        if translation.y + 2. < water_height {
            commands.entity(entity).despawn_recursive();
        }
    }
}
