use crate::artillery::components::{Artillery, ArtilleryAiming, ArtilleryReady, Projectile};
use crate::artillery::resources::{EndAimArtilleryAnimationClips, StartAimArtilleryAnimationClips};
use crate::artillery::{
    PORT_BACK_CANNON_TAG, PORT_FRONT_CANNON_TAG, STARBOARD_BACK_CANNON_TAG,
    STARBOARD_FRONT_CANNON_TAG,
};
use crate::assets::resources::ModelAssets;
use crate::connection::systems::RollbackConfig;
use crate::inputs::fire;
use crate::physics::bundles::ParticleBundle;
use crate::physics::components::Velocity;
use crate::player::components::Player;
use crate::utils::linear_algebra::is_facing;
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::time::Duration;

// Check https://johanhelsing.studio/posts/extreme-bevy-3
// Add this in the rollback schedule (if a bullet fired by the other player was mis-predicted, this
// is obviously something we’d want to correct!)
pub fn fire_artillery(
    mut commands: Commands,
    inputs: Res<PlayerInputs<RollbackConfig>>,
    mut player_query: Query<(&mut ArtilleryReady, &Transform, &Player)>,
    model_assets: Res<ModelAssets>,
) {
    // TODO: Query for artillery. Sort out what cannons are to be fired. Use muzzle_velocity for projectiles.
    for (mut artillery_ready, transform, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        if fire(input) && artillery_ready.0 {
            commands
                .spawn((
                    SceneBundle {
                        scene: model_assets.scene_handles["cannon_ball.glb"].clone(),
                        transform: Transform::from_translation(transform.translation),
                        ..default()
                    },
                    Name::new("Projectile"),
                    Projectile,
                    ParticleBundle {
                        velocity: Velocity(Vec3::Y * 18.),
                        ..default()
                    },
                ))
                .add_rollback();

            artillery_ready.0 = false;
        }
    }
}

pub fn reload_artillery(
    inputs: Res<PlayerInputs<RollbackConfig>>,
    mut player_query: Query<(&mut ArtilleryReady, &Player)>,
) {
    for (mut artillery_ready, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        if !fire(input) && !artillery_ready.0 {
            artillery_ready.0 = true;
        }
    }
}

// Continue from https://github.com/claudijo/pirate-sea-jam/blob/infinite-ocean/src/systems/artillery.rs
pub fn start_aim_artillery(
    mut artillery_query: Query<(
        &GlobalTransform,
        &Name,
        &mut Artillery,
        &mut AnimationPlayer,
    )>,
    children_query: Query<&Children>,
    mut player_query: Query<(Entity, &mut ArtilleryAiming, &Player)>,
    inputs: Res<PlayerInputs<RollbackConfig>>,
    animation_clips: Res<StartAimArtilleryAnimationClips>,
) {
    let dummy_closest_target = Vec3::ZERO;

    for (ship_entity, mut artillery_aiming, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        if fire(input) && !artillery_aiming.0 {
            println!("Start aiming");
            artillery_aiming.0 = true;

            for descendant in children_query.iter_descendants(ship_entity) {
                if let Ok((global_transform, name, mut artillery, mut animation_player)) =
                    artillery_query.get_mut(descendant)
                {
                    if is_facing(
                        global_transform.left(),
                        global_transform.translation(),
                        dummy_closest_target,
                    ) {
                        if let Some(animation_clip_handle) =
                            animation_clips.handles.get(name.as_str())
                        {
                            artillery.is_aiming = true;

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

pub fn stop_aim_and_fire_artillery(
    inputs: Res<PlayerInputs<RollbackConfig>>,
    mut player_query: Query<(&mut ArtilleryAiming, &Player)>,
    animation_clips: Res<EndAimArtilleryAnimationClips>,
) {
    for (mut artillery_aiming, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        if !fire(input) && artillery_aiming.0 {
            println!("stop aiming and fire");
            artillery_aiming.0 = false;
        }
    }
}

pub fn register_start_aim_artillery_animations(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let keyframe_timestamps = vec![0.0, 0.6, 1.2];

    let mut animation_clip_handles = HashMap::new();

    let tilt_port_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![
            Quat::from_rotation_y(PI) * Quat::from_rotation_z(0.),
            Quat::from_rotation_y(PI) * Quat::from_rotation_z(-30_f32.to_radians()),
            Quat::from_rotation_y(PI) * Quat::from_rotation_z(0.),
        ]),
    };

    let tilt_starboard_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![
            Quat::from_rotation_z(0.),
            Quat::from_rotation_z(-30_f32.to_radians()),
            Quat::from_rotation_z(0.),
        ]),
    };

    for (tag, tilt_curve) in [
        (
            PORT_BACK_CANNON_TAG,
            tilt_port_cannon_animation_curve.clone(),
        ),
        (
            PORT_FRONT_CANNON_TAG,
            tilt_port_cannon_animation_curve.clone(),
        ),
        (
            STARBOARD_BACK_CANNON_TAG,
            tilt_starboard_cannon_animation_curve.clone(),
        ),
        (
            STARBOARD_FRONT_CANNON_TAG,
            tilt_starboard_cannon_animation_curve.clone(),
        ),
    ] {
        let mut animation_clip = AnimationClip::default();

        animation_clip.add_curve_to_path(
            EntityPath {
                parts: vec![Name::new(tag)],
            },
            tilt_curve,
        );

        animation_clip_handles.insert(tag, animations.add(animation_clip));
    }

    commands.insert_resource(StartAimArtilleryAnimationClips {
        handles: animation_clip_handles,
    });
}

pub fn register_stop_aim_artillery_animations(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let keyframe_timestamps = vec![0.0];

    let mut animation_clip_handles = HashMap::new();

    let reset_tilt_port_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![Quat::from_rotation_y(PI)]),
    };

    let reset_tilt_starboard_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![Quat::from_rotation_y(0.)]),
    };

    let stretch_and_squash_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: vec![0.0, 0.06, 0.18, 0.2],
        keyframes: Keyframes::Scale(vec![
            Vec3::new(1., 1., 1.),
            Vec3::new(0.6, 1.6, 1.6),
            Vec3::new(1.1, 0.9, 0.9),
            Vec3::new(1., 1., 1.),
        ]),
    };

    for (tag, reset_tilt_curve) in [
        (
            PORT_BACK_CANNON_TAG,
            reset_tilt_port_cannon_animation_curve.clone(),
        ),
        (
            PORT_FRONT_CANNON_TAG,
            reset_tilt_port_cannon_animation_curve.clone(),
        ),
        (
            STARBOARD_BACK_CANNON_TAG,
            reset_tilt_starboard_cannon_animation_curve.clone(),
        ),
        (
            STARBOARD_FRONT_CANNON_TAG,
            reset_tilt_starboard_cannon_animation_curve.clone(),
        ),
    ] {
        let mut animation_clip = AnimationClip::default();

        // Reset tilt
        animation_clip.add_curve_to_path(
            EntityPath {
                parts: vec![Name::new(tag)],
            },
            reset_tilt_curve,
        );

        // Stretch and squash
        animation_clip.add_curve_to_path(
            EntityPath {
                parts: vec![Name::new(tag)],
            },
            stretch_and_squash_cannon_animation_curve.clone(),
        );

        animation_clip_handles.insert(tag, animations.add(animation_clip));
    }

    commands.insert_resource(EndAimArtilleryAnimationClips {
        handles: animation_clip_handles,
    });
}
