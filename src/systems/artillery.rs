use crate::components::cannon::{Aim, Cannon, CannonBall};
use crate::components::ship::{PlayerShip, Ship};
use crate::components::shooting_target::ShootingTarget;
use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::plugins::assets::ModelAssets;
use crate::resources::wave::Wave;
use crate::systems::ship::{EndAimCannonAnimationClips, StartAimCannonAnimationClips};
use crate::utils::targeting;
use bevy::animation::RepeatAnimation;
use bevy::prelude::*;
use bevy_rapier3d::geometry::ColliderMassProperties::Density;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::time::Duration;

pub fn handle_cannon_aim_event(
    shooting_target_query: Query<&Transform, With<ShootingTarget>>,
    ship_query: Query<&Transform, With<Ship>>,
    mut cannon_query: Query<(
        &mut Aim,
        &mut AnimationPlayer,
        &Name,
        &GlobalTransform,
        &Cannon,
    )>,
    mut aim_cannon_event_reader: EventReader<AimCannonEvent>,
    animation_clips: Res<StartAimCannonAnimationClips>,
) {
    for event in aim_cannon_event_reader.read() {
        if let Ok(ship_transform) = ship_query.get(**event) {
            let target_translations = shooting_target_query
                .iter()
                .map(|transform| transform.translation)
                .collect();

            if let Some(closest_target) =
                targeting::find_closest_target(&ship_transform.translation, &target_translations)
            {
                for (mut aim, mut animation_player, name, global_transform, cannon) in
                    &mut cannon_query
                {
                    if cannon.rig != **event {
                        continue;
                    }

                    let target_direction = *closest_target - global_transform.translation();
                    if global_transform.left().dot(target_direction) > 0. {
                        aim.is_targeting = true;

                        if let Some(animation_clip_handle) =
                            animation_clips.handles.get(name.as_str())
                        {
                            animation_player
                                .play_with_transition(
                                    animation_clip_handle.clone_weak(),
                                    Duration::from_secs(0.6 as u64),
                                )
                                .repeat();
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
    mut cannon_query: Query<
        (
            &GlobalTransform,
            &mut Aim,
            &mut AnimationPlayer,
            &Name,
            &Cannon,
        ),
        Without<Ship>,
    >,
    mut fire_cannon_event_reader: EventReader<FireCannonEvent>,
    mut ship_query: Query<(&Velocity, &mut ExternalImpulse), With<PlayerShip>>,
    animation_clips: Res<EndAimCannonAnimationClips>,
) {
    for event in fire_cannon_event_reader.read() {
        let mut rng = rand::thread_rng();

        for (global_transform, mut aim, mut animation_player, name, cannon) in &mut cannon_query {
            if **event != cannon.rig {
                continue;
            }

            if aim.is_targeting {
                aim.is_targeting = false;

                if let Some(animation_clip_handle) = animation_clips.handles.get(name.as_str()) {
                    animation_player
                        .set_repeat(RepeatAnimation::Never)
                        .play_with_transition(
                            animation_clip_handle.clone_weak(),
                            Duration::from_secs(1.2 as u64),
                        );
                }

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

pub fn despawn_cannon_ball(
    mut commands: Commands,
    cannon_ball_query: Query<(Entity, &GlobalTransform), With<CannonBall>>,
    wave_machine: Res<Wave>,
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
