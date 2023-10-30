use crate::components::cannon::{Aim, Cannon, CannonBall, Tilt};
use crate::resources::wave_machine::WaveMachine;
use bevy::prelude::*;
use bevy_rapier3d::geometry::ColliderMassProperties::Density;
use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::resources::assets::ModelAssets;
use bevy_rapier3d::prelude::*;
use crate::components::ship::Ship;
use crate::components::shooting_target::ShootingTarget;
use crate::utils::targeting;
use rand::Rng;

// Will start aiming cannons facing closes target
pub fn handle_cannon_aim_event(
    shooting_target_query: Query<&Transform, With<ShootingTarget>>,
    ship_query: Query<&Transform, With<Ship>>,
    mut cannon_query: Query<(Entity, &mut Aim, &GlobalTransform, &Cannon)>,
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
                for (entity, mut aim, transform, cannon) in &mut cannon_query {
                    if let Some(rig) = cannon.rig {
                        if rig != event.source {
                            continue;
                        }

                        let target_direction = *closest_target - transform.translation();
                        if transform.left().dot(target_direction) > 0. {
                            aim.is_targeting = true;

                            println!("Aim cannon {:?}", entity);
                        }
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
    mut cannon_query: Query<(Entity, &GlobalTransform, &mut Aim, &Cannon)>,
    mut event_reader: EventReader<FireCannonEvent>,
    mut ship_query: Query<(&Velocity, &mut ExternalImpulse), With<Ship>>,
) {
    for event in event_reader.iter() {
        let mut rng = rand::thread_rng();

        for (entity, global_transform, mut aim, cannon) in &mut cannon_query {
            if let Some(rig) = cannon.rig {
                if event.source != rig {
                    continue;
                }

                if aim.is_targeting {
                    aim.is_targeting = false;

                    if let Ok((ship_velocity, mut external_impulse)) = ship_query.get_mut(rig) {
                        // Make ship recoil
                        let recoil_scale = if cannon.tilt_factor > 0. { -10. } else { 10. };
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
}

pub fn rewind_cannon_tilt(
    mut cannons: Query<(&mut Transform, &mut Tilt, &Aim, &Cannon)>,
    time: Res<Time>,
) {
    for (mut transform, mut barrel_tilt, carriage, cannon) in &mut cannons {
        if !carriage.is_targeting && barrel_tilt.angle != 0. {
            let angle = barrel_tilt.angle + time.delta_seconds() * cannon.tilt_factor * -2.;

            if cannon.tilt_factor > 0. {
                barrel_tilt.angle = angle.max(0.);
            } else {
                barrel_tilt.angle = angle.min(0.);
            }

            transform.rotation =
                Quat::from_rotation_z(cannon.default_tilt + barrel_tilt.angle.to_radians());
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
