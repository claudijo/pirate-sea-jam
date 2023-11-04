use crate::components::cannon::{Aim, Cannon, Tilt};
use crate::components::pontoon::Pontoon;
use crate::components::ship::{
    PlayerShip, Ship, ShipBooster, ShipFlag, ShipHelm, ShipRudder, ShipSail,
};
use crate::events::game::RestartGameEvent;
use crate::resources::assets::ModelAssets;
use crate::resources::despawn::ShipDespawnEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

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
            PlayerShip,
            Ship { ..default() },
            ShipBooster { ..default() },
            ShipRudder { ..default() },
        ))
        .with_children(|child_builder| {
            // Colliders
            child_builder.spawn((Collider::cuboid(0.8, 0.5, 2.),));
        })
        .id();

    // Spawn children that need a reference to the parent entity
    commands
        .entity(parent_entity)
        .with_children(|child_builder| {
            // Models
            child_builder
                .spawn(SceneBundle {
                    scene: model_assets.scene_handles["medium_hull.glb"].clone(),
                    transform: Transform::from_xyz(0., -0.5, 0.),
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder.spawn((
                        ShipHelm,
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_helm.glb"].clone(),
                            transform: Transform::from_xyz(0., 5.5806, -1.0694),
                            ..default()
                        },
                    ));

                    child_builder.spawn((
                        ShipSail,
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_pirate_sail.glb"].clone(),
                            transform: Transform::from_xyz(0., 2.3248, 1.3574),
                            ..default()
                        },
                    ));

                    child_builder.spawn((
                        ShipFlag { rig: parent_entity },
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_flag.glb"].clone(),
                            transform: Transform::from_xyz(0., 9.38793, 1.35834),
                            ..default()
                        },
                    ));

                    let cannons = [
                        ([1.1769, 1.4593, -0.5485], PI),    // Port back cannon
                        ([1.13846, 1.54822, 1.54781], PI),  // Port front cannon
                        ([-1.1769, 1.4593, -0.5485], 0.),   // Starboard back cannon
                        ([-1.13846, 1.54822, 1.54781], 0.), // Starboard front cannon
                    ];

                    for (cannon_transform, cannon_y_rotation) in cannons {
                        child_builder.spawn((
                            Aim { ..default() },
                            Tilt {
                                acceleration: 3.,
                                velocity: 0.,
                                stabilize_tilt_timer: Timer::from_seconds(0.4, TimerMode::Once),
                            },
                            Cannon {
                                rig: parent_entity,
                                power: 1.,
                                max_tilt: 40.,
                            },
                            SceneBundle {
                                scene: model_assets.scene_handles["medium_canon.glb"].clone(),
                                transform: Transform::from_translation(Vec3::from_array(
                                    cannon_transform,
                                ))
                                .with_rotation(Quat::from_rotation_y(cannon_y_rotation)),
                                ..default()
                            },
                        ));
                    }
                });
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

pub fn reset_ship(
    model_assets: Res<ModelAssets>,
    ship_query: Query<Entity, With<Ship>>,
    mut commands: Commands,
    mut ship_despawn: ResMut<ShipDespawnEntities>,
    mut restart_game_event_reader: EventReader<RestartGameEvent>,
) {
    if restart_game_event_reader.is_empty() {
        return;
    }
    restart_game_event_reader.clear();

    // Note that some joint related child entities seem to be missing from the normal
    // parent-child-hierarchy when despawning, so those are registered and handled "manually".
    // (See https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier3d/examples/joints_despawn3.rs)
    for parent in &ship_query {
        for entity in &ship_despawn.entities {
            commands.entity(*entity).despawn();
        }

        ship_despawn.entities.clear();
        commands.entity(parent).despawn_recursive();
    }

    // Spawn new ship
    spawn_ship(commands, model_assets, ship_despawn);
}
