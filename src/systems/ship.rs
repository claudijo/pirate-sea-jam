use crate::components::cannon::Cannon;
use crate::components::pontoon::Pontoon;
use crate::components::ship::{Booster, Helm, Pennant, Sail, Ship, TurnRate};
use crate::resources::assets::ModelAssets;
use crate::resources::despawn::ShipDespawnEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

const CANNON_POWER: f32 = 1.;

pub fn spawn_ship(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    mut ship_despawn: ResMut<ShipDespawnEntities>,
) {
    let parent_entity = commands
        // Rigid body
        .spawn((
            TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
            RigidBody::Dynamic,
            VisibilityBundle { ..default() }, // Necessary to display child scene bundle
            ExternalImpulse { ..default() },
            ExternalForce { ..default() },
            Velocity { ..default() },
            Damping {
                angular_damping: 20.,
                linear_damping: 4.,
            },
            Ship { ..default() },
            Booster { ..default() },
            TurnRate { ..default() },
        ))
        .with_children(|child_builder| {
            // Colliders
            child_builder.spawn((
                Collider::cuboid(0.8, 0.5, 2.),
            ));

            // Models
            child_builder
                .spawn(SceneBundle {
                    scene: model_assets.scene_handles["medium_hull"].clone(),
                    transform: Transform::from_xyz(0., -0.5, 0.),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Helm,
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_helm"].clone(),
                            transform: Transform::from_xyz(0., 5.5806, -1.0694),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Sail,
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_pirate_sail"].clone(),
                            transform: Transform::from_xyz(0., 2.3248, 1.3574),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    // Spawn children that need a reference to the parent entity
    commands
        .entity(parent_entity)
        .with_children(|child_builder| {
            child_builder.spawn((
                Pennant {
                    rig: Some(parent_entity),
                },
                SceneBundle {
                    scene: model_assets.scene_handles["medium_flag"].clone(),
                    transform: Transform::from_xyz(0., 9.38793, 1.35834),
                    ..default()
                },
            ));

            child_builder.spawn((
                Cannon {
                    rig: Some(parent_entity),
                    power: CANNON_POWER,
                    ..default()
                },
                SceneBundle {
                    scene: model_assets.scene_handles["port_back_canon"].clone(),
                    transform: Transform::from_xyz(1.1769, 1.4593, -0.5485)
                        .with_rotation(Quat::from_rotation_z(PI)),
                    ..default()
                },
            ));

            child_builder.spawn((
                Cannon {
                    rig: Some(parent_entity),
                    power: CANNON_POWER,
                    ..default()
                },
                SceneBundle {
                    scene: model_assets.scene_handles["port_front_canon"].clone(),
                    transform: Transform::from_xyz(1.13846, 1.54822, 1.54781)
                        .with_rotation(Quat::from_rotation_z(PI)),
                    ..default()
                },
            ));

            child_builder.spawn((
                Cannon {
                    rig: Some(parent_entity),
                    power: CANNON_POWER,
                    ..default()
                },
                SceneBundle {
                    scene: model_assets.scene_handles["starboard_back_canon"].clone(),
                    transform: Transform::from_xyz(-1.1769, 1.4593, -0.5485),
                    ..default()
                },
            ));

            child_builder.spawn((
                Cannon {
                    rig: Some(parent_entity),
                    power: CANNON_POWER,
                    ..default()
                },
                SceneBundle {
                    scene: model_assets.scene_handles["starboard_front_canon"].clone(),
                    transform: Transform::from_xyz(-1.13846, 1.54822, 1.54781),
                    ..default()
                },
            ));
        });

    let pontoon_positions = [
        [-0.8, 0., 2.],
        [0.8, 0., 2.],
        [-1., 0., 0.],
        [1., 0., 0.],
        [-0.6, 0., -2.],
        [0.6, 0., -2.],
    ];

    let pontoon_radius = 0.5;

    for pontoon_position in pontoon_positions {
        let position = Vec3::from_array(pontoon_position);
        let joint = FixedJointBuilder::new().local_anchor1(position);

        let child_pontoon = commands
            .spawn((
                TransformBundle::from(Transform::from_translation(position)),
                RigidBody::Dynamic,
                Collider::ball(pontoon_radius),
                ExternalForce { ..default() },
                Damping { ..default() },
                Velocity { ..default() },
                CollisionGroups::new(Group::NONE, Group::NONE),
                Pontoon {
                    radius: pontoon_radius,
                    ..default()
                },
                ImpulseJoint::new(parent_entity, joint),
            ))
            .id();

        // Need to add pontoon to registry for later despawn
        ship_despawn.entities.push(child_pontoon);
    }
}
