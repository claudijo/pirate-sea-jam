use crate::args::resources::Args;
use crate::artillery::components::{Artillery, ArtilleryAiming, ArtilleryReady};
use crate::artillery::{
    PORT_BACK_CANNON_TAG, PORT_FRONT_CANNON_TAG, STARBOARD_BACK_CANNON_TAG,
    STARBOARD_FRONT_CANNON_TAG,
};
use crate::assets::resources::ModelAssets;
use crate::connection::systems::RollbackConfig;
use crate::controls::components::{Controls, SailTrimRatio, WheelTurnRatio};
use crate::inputs::turn_action_from_input;
use crate::physics::bundles::{ParticleBundle, SpindleBundle};
use crate::physics::components::{
    Aerofoil, AngularDamping, Area, Buoy, Hydrofoil, Inertia, LinearDamping, LinearVelocity, Mass,
    Rudder, SailTrim,
};
use crate::player::components::{Flag, Player, Wheel};
use crate::player::{WHEEL_TURN_ACCELERATION, WHEEL_TURN_DAMPING};
use crate::utils::linear_algebra::face_normal;
use crate::wind::resources::Wind;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs, Rollback};
use std::f32::consts::{E, PI};

pub fn spawn_players(
    mut commands: Commands,
    args: Res<Args>,
    model_assets: Res<ModelAssets>,
    mut assets: ResMut<Assets<Mesh>>,
) {
    let placement_circle_radius = 5.;
    for handle in 0..args.num_players {
        let placement_angle = handle as f32 / args.num_players as f32 * 2. * PI;
        let x = placement_circle_radius * placement_angle.cos();
        let z = placement_circle_radius * placement_angle.sin();

        // Duplicate vertices once for flag mesh here, which will facilitate recalculating normals
        // when animating the flag later on, even if accessing the mesh through the scene asset
        let flag_mesh_handle = &model_assets.mesh_handles["medium_flag.glb"];
        let flag_mesh = assets.get_mut(flag_mesh_handle).unwrap();
        flag_mesh.duplicate_vertices();

        commands
            .spawn((
                SpatialBundle::from_transform(
                    Transform::from_translation(Vec3::new(x, 0., z))
                        .with_rotation(Quat::from_rotation_y(4. * PI / 8.)),
                ),
                Player { handle },
                Controls::default(),
                WheelTurnRatio::default(),
                SailTrimRatio::default(),
                ArtilleryReady::default(),
                ArtilleryAiming::default(),
                Name::new("Ship"),
                SpindleBundle {
                    inertia: Inertia::cuboid(4., 3., 3., 100.),
                    angular_damping: AngularDamping(0.6),
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
                                Wheel,
                                Name::new("Wheel"),
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
                                SailTrim,
                            ))
                            .add_rollback();

                        // Place the force generating sail in center of gravity so that we don't
                        // generate any torque, which messes things up. Same with keel.
                        child_builder
                            .spawn((
                                TransformBundle::from_transform(Transform::from_rotation(
                                    Quat::from_rotation_y(PI / 4.),
                                )),
                                Name::new("Virtual sail"),
                                Area(8.),
                                Aerofoil,
                                SailTrim,
                            ))
                            .add_rollback();

                        child_builder
                            .spawn((
                                TransformBundle::default(),
                                Name::new("Keel"),
                                Area(1.),
                                Hydrofoil,
                            ))
                            .add_rollback();

                        child_builder
                            .spawn((
                                TransformBundle::from_transform(Transform::from_xyz(0., -1., -2.)),
                                Name::new("Rudder"),
                                Area(0.05),
                                Rudder,
                                Hydrofoil,
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
                                        muzzle_velocity: 24.,
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

pub fn animate_wheel_turn(
    player_query: Query<&WheelTurnRatio, With<Rollback>>,
    mut helm_query: Query<&mut Transform, With<Wheel>>,
) {
    for wheel_turn_ratio in &player_query {
        for mut transform in &mut helm_query {
            transform.rotation = Quat::from_rotation_z(wheel_turn_ratio.0 * 4.);
        }
    }
}

pub fn update_rudder(
    player_query: Query<&WheelTurnRatio, With<Rollback>>,
    mut rudder_query: Query<&mut Transform, With<Rudder>>,
) {
    for wheel_turn_ratio in &player_query {
        for mut transform in &mut rudder_query {
            transform.rotation = Quat::from_rotation_y(wheel_turn_ratio.0 * PI / 8.);
        }
    }
}

pub fn animate_sail_trim(
    player_query: Query<&SailTrimRatio, With<Rollback>>,
    mut sail_query: Query<&mut Transform, With<SailTrim>>,
) {
    for sail_trim_ratio in &player_query {
        for mut transform in &mut sail_query {
            transform.rotation = Quat::from_rotation_y(sail_trim_ratio.0 * PI / 4.);
        }
    }
}

pub fn update_hull_drag(
    mut player_query: Query<(&mut LinearDamping, &GlobalTransform, &LinearVelocity)>,
) {
    for (mut linear_damping, global_transform, linear_velocity) in &mut player_query {
        if global_transform.back().dot(linear_velocity.0) > 0. {
            // Going forward
            linear_damping.0 = 0.8;
        } else {
            // Going backwards
            linear_damping.0 = 0.6;
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

pub fn update_wheel_turn_ratio(
    mut player_query: Query<(&mut WheelTurnRatio, &Controls), With<Rollback>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mut wheel_turn_ratio, controls) in &mut player_query {
        wheel_turn_ratio.0 += controls.turn_action as f32 * WHEEL_TURN_ACCELERATION * delta_time;
        wheel_turn_ratio.0 *= WHEEL_TURN_DAMPING.powf(delta_time);
        wheel_turn_ratio.0 = wheel_turn_ratio.0.clamp(-1., 1.);
    }
}

pub fn update_sail_trim_ratio(
    mut player_query: Query<(&mut SailTrimRatio, &GlobalTransform), With<Rollback>>,
    wind: Res<Wind>,
) {
    for (mut sail_trim_ratio, global_transform) in &mut player_query {
        let trim_ratio = global_transform
            .forward()
            .xz()
            .angle_between(wind.0.xz())
            .sin();
        sail_trim_ratio.0 = trim_ratio;
    }
}
