use crate::args::resources::Args;
use crate::artillery::components::{Artillery, ArtilleryAiming, ArtilleryReady};
use crate::artillery::{
    PORT_BACK_CANNON_TAG, PORT_FRONT_CANNON_TAG, STARBOARD_BACK_CANNON_TAG,
    STARBOARD_FRONT_CANNON_TAG,
};
use crate::assets::resources::ModelAssets;
use crate::connection::systems::RollbackConfig;
use crate::floating_body::components::{
    Controls, FloatingLinearVelocity, FloatingPosition, Yaw, YawRotationalSpeed,
};
use crate::inputs::turn_action_from_input;
use crate::ocean::resources::Wave;
use crate::physics::bundles::{ParticleBundle, SpindleBundle};
use crate::physics::components::{
    AerofoilArea, AngularDamping, AngularVelocity, Buoy, Inertia, LinearDamping, Mass,
};
use crate::player::components::{Flag, Helm, Player};
use crate::player::{
    ANGULAR_ACCELERATION, ANGULAR_DAMPING, LINEAR_ACCELERATION, LINEAR_DAMPING, MAX_ANGULAR_SPEED,
    MAX_LINEAR_SPEED, TRACTION,
};
use crate::utils::f32_extensions::F32Ext;
use crate::utils::linear_algebra::face_normal;
use crate::utils::vec2_extensions::Vec2Ext;
use crate::wind::resources::Wind;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs, Rollback};
use std::f32::consts::{E, PI, TAU};

pub fn spawn_players(
    mut commands: Commands,
    args: Res<Args>,
    model_assets: Res<ModelAssets>,
    mut assets: ResMut<Assets<Mesh>>,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let placement_circle_radius = 5.;
    for handle in 0..args.num_players {
        let placement_angle = handle as f32 / args.num_players as f32 * 2. * PI;
        let x = placement_circle_radius * placement_angle.cos();
        let z = placement_circle_radius * placement_angle.sin();

        // Duplicate vertices once for flag mesh here, which will facilitate recalculating normals when animating the
        // flag later on, even if accessing the mesh through the scene asset
        let flag_mesh_handle = &model_assets.mesh_handles["medium_flag.glb"];
        let flag_mesh = assets.get_mut(flag_mesh_handle).unwrap();
        flag_mesh.duplicate_vertices();

        commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(Vec3::new(x, 0., z))),
                Player { handle },
                FloatingPosition(Vec2::new(x, z)),
                Yaw::default(),
                Controls::default(),
                FloatingLinearVelocity::default(),
                YawRotationalSpeed::default(),
                ArtilleryReady::default(),
                ArtilleryAiming::default(),
                Name::new("Ship"),
                // LinearDrag::default(),
                // AngularDrag::default(),
                SpindleBundle {
                    inertia: Inertia::cuboid(4., 3., 3., 100.),
                    angular_damping: AngularDamping(0.8),
                    ..default()
                },
                ParticleBundle {
                    mass: Mass(100.),
                    linear_damping: LinearDamping(0.8),
                    ..default()
                },
            ))
            .with_children(|child_builder| {
                for buoy_translation in [
                    Vec3::new(1.25, 0.5, 1.25),
                    Vec3::new(-1.25, 0.5, 1.25),
                    Vec3::new(1.25, 0.5, -1.25),
                    Vec3::new(-1.25, 0.5, -1.25),
                ] {
                    child_builder
                        .spawn((
                            TransformBundle::from_transform(Transform::from_translation(
                                buoy_translation,
                            )),
                            Buoy {
                                volume: 0.75,
                                max_depth: 0.5,
                                ..default()
                            },
                        ))
                        .add_rollback();
                }

                child_builder
                    .spawn((
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_hull.glb"].clone(),
                            transform: Transform::from_xyz(0., 0., 0.),
                            ..default()
                        },
                        Name::new("Hull"),
                    ))
                    .add_rollback()
                    .with_children(|child_builder| {
                        child_builder
                            .spawn((
                                SceneBundle {
                                    scene: model_assets.scene_handles["medium_helm.glb"].clone(),
                                    transform: Transform::from_xyz(0., 5.5806, -1.0694),
                                    ..default()
                                },
                                Helm,
                                Name::new("Helm"),
                            ))
                            .add_rollback();

                        child_builder
                            .spawn((
                                SceneBundle {
                                    scene: model_assets.scene_handles["medium_pirate_sail.glb"]
                                        .clone(),
                                    transform: Transform::from_xyz(0., 2.3248, 1.3574),
                                    ..default()
                                },
                                Name::new("Sail"),
                                AerofoilArea(0.01),
                            ))
                            .add_rollback();

                        child_builder
                            .spawn((
                                SceneBundle {
                                    scene: model_assets.scene_handles["medium_flag.glb"].clone(),
                                    transform: Transform::from_xyz(0., 9.38793, 1.35834),
                                    ..default()
                                },
                                Flag,
                                Name::new("Flag"),
                            ))
                            .add_rollback();

                        #[rustfmt::skip]
                            let cannons = [
                            ([1.1769, 1.4593, -0.5485], PI, PORT_BACK_CANNON_TAG),
                            ([1.13846, 1.54822, 1.54781], PI, PORT_FRONT_CANNON_TAG),
                            ([-1.1769, 1.4593, -0.5485], 0., STARBOARD_BACK_CANNON_TAG),
                            ([-1.13846, 1.54822, 1.54781], 0., STARBOARD_FRONT_CANNON_TAG),
                        ];

                        for (cannon_transform, cannon_y_rotation, name) in cannons {
                            child_builder
                                .spawn((
                                    SceneBundle {
                                        scene: model_assets.scene_handles["medium_canon.glb"]
                                            .clone(),
                                        transform: Transform::from_translation(Vec3::from_array(
                                            cannon_transform,
                                        ))
                                        .with_rotation(Quat::from_rotation_y(cannon_y_rotation)),
                                        ..default()
                                    },
                                    Artillery {
                                        muzzle_velocity: 18.,
                                        ..default()
                                    },
                                    AnimationPlayer::default(),
                                    Name::new(name),
                                ))
                                .add_rollback();
                        }
                    });
            })
            // Add a Rollback component with a unique id
            .add_rollback();
    }
}

const AMPLITUDE: f32 = 0.4;
const WAVE_LENGTH: f32 = 3.;
const WAVE_SPEED: f32 = -8.;
const WAVE_NUMBER: f32 = 2. * PI / WAVE_LENGTH;

// https://www.rorydriscoll.com/2016/03/07/frame-rate-independent-damping-using-lerp/

pub fn animate_flag(
    mut flag_query: Query<(Entity, &mut Transform, &Parent), With<Flag>>,
    wind: Res<Wind>,
    children_query: Query<&Children>,
    global_transform_query: Query<&GlobalTransform>,
    mesh_query: Query<&Handle<Mesh>>,
    mut assets: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_seconds();

    for (flag_entity, mut flag_transform, flag_parent) in &mut flag_query {
        // Rotate flag from wind
        let flag_parent_global_transform = global_transform_query.get(flag_parent.get());
        if let Ok(flag_parent_global_transform) = flag_parent_global_transform {
            let angle = wind
                .0
                .xz()
                .angle_between(flag_parent_global_transform.forward().xz());
            flag_transform.rotation = Quat::from_rotation_y(angle);
        }

        for child_entity in children_query.iter_descendants(flag_entity) {
            if let Ok(handle) = mesh_query.get(child_entity) {
                let mesh = assets.get_mut(handle).unwrap();
                let mut positions: Vec<[f32; 3]> = Vec::new();
                let mut normals: Vec<[f32; 3]> = Vec::new();

                let triangles = mesh
                    .attribute(Mesh::ATTRIBUTE_POSITION)
                    .unwrap()
                    .as_float3()
                    .expect(
                        "`Mesh::ATTRIBUTE_POSITION` vertex attributes should be of type `float3`",
                    )
                    .chunks_exact(3);

                for vertices in triangles {
                    let flat_normal = face_normal(vertices[0], vertices[1], vertices[2]);

                    for vertex in vertices {
                        let wave_phase = WAVE_NUMBER * (vertex[2] - WAVE_SPEED * elapsed_time);
                        let adjusted_amplitude =
                            0_f32.lerp(AMPLITUDE, 1. - E.powf(-vertex[2].abs()));

                        // https://catlikecoding.com/unity/tutorials/flow/waves/
                        let position =
                            [adjusted_amplitude * wave_phase.sin(), vertex[1], vertex[2]];
                        let tangent =
                            Vec3::new(WAVE_NUMBER * adjusted_amplitude * wave_phase.cos(), 0., 1.)
                                .normalize();
                        let mut normal = Vec3::new(tangent.z, 0., -tangent.x);

                        // Ensure normal is not inverted by comparing it with the computed flat normal
                        if normal.x * flat_normal[0] < 0. {
                            normal *= -1.;
                        }

                        positions.push(position);
                        normals.push(normal.into());
                    }
                }

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            }
        }
    }
}

pub fn animate_helm(
    player_query: Query<&YawRotationalSpeed, With<Rollback>>,
    mut helm_query: Query<&mut Transform, With<Helm>>,
) {
    for yaw_rotational_speed in &player_query {
        for mut transform in &mut helm_query {
            transform.rotation = Quat::from_rotation_z(yaw_rotational_speed.0 * TAU);
        }
    }
}

pub fn apply_inputs(
    mut player_query: Query<(&mut Controls, &Player), With<Rollback>>,
    inputs: Res<PlayerInputs<RollbackConfig>>,
) {
    for (mut controls, player) in &mut player_query {
        controls.turn_action = turn_action_from_input(inputs[player.handle]);
        controls.accelerate_action = 1; // Always(?) sail full speed ahead
    }
}

// Take control component and calculate new velocity and update velocity component
pub fn update_player_velocity(
    mut player_query: Query<
        (
            &mut FloatingLinearVelocity,
            &mut YawRotationalSpeed,
            &Yaw,
            &Controls,
        ),
        With<Rollback>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mut linear_velocity, mut rotational_speed, yaw, controls) in &mut player_query {
        linear_velocity.0 +=
            yaw.forward() * controls.accelerate_action as f32 * LINEAR_ACCELERATION * delta_time;
        linear_velocity.0 *= LINEAR_DAMPING.powf(delta_time);
        linear_velocity.0 = linear_velocity.0.clamp_length_max(MAX_LINEAR_SPEED);

        linear_velocity.0 = linear_velocity
            .0
            .normalize()
            .damp(yaw.forward(), TRACTION, delta_time)
            * linear_velocity.0.length();

        let rotation_speed_factor = linear_velocity.0.length() / MAX_LINEAR_SPEED;
        rotational_speed.0 +=
            controls.turn_action as f32 * ANGULAR_ACCELERATION * rotation_speed_factor * delta_time;
        rotational_speed.0 *= ANGULAR_DAMPING.powf(delta_time);
        rotational_speed.0 = rotational_speed
            .0
            .clamp(-MAX_ANGULAR_SPEED, MAX_ANGULAR_SPEED);
    }
}

#[allow(dead_code)]
pub fn debug_velocity(
    player_query: Query<(&Transform, &Yaw, &FloatingLinearVelocity), With<Rollback>>,
    mut gizmos: Gizmos,
) {
    for (transform, yaw, linear_velocity) in &player_query {
        gizmos.ray(
            transform.translation + Vec3::new(0., 2., 0.),
            yaw.forward().extend_with_y(0.) * 8.,
            Color::BLUE,
        );
        gizmos.ray(
            transform.translation + Vec3::new(0., 2., 0.),
            linear_velocity.0.extend_with_y(0.) * 1.4,
            Color::GREEN,
        );
    }
}

pub fn update_player_position(
    mut player_query: Query<
        (
            &mut Yaw,
            &mut FloatingPosition,
            &FloatingLinearVelocity,
            &YawRotationalSpeed,
        ),
        With<Rollback>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mut yaw, mut position, linear_velocity, rotational_speed) in &mut player_query {
        yaw.0 -= rotational_speed.0 * delta_time;
        position.0 += linear_velocity.0 * delta_time;
    }
}
