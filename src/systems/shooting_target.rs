use crate::components::pontoon::Pontoon;
use crate::components::ship::Pennant;
use crate::components::shooting_target::ShootingTarget;
use crate::resources::assets::ModelAssets;
use crate::resources::despawn::ShootingTargetDespawnEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_shooting_target(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    mut shooting_target_despawn: ResMut<ShootingTargetDespawnEntities>,
) {
    let spawn_at = Vec3::new(10., 0., 10.);

    let parent_entity = commands
        // Rigid body
        .spawn((
            TransformBundle::from(Transform::from_translation(spawn_at)),
            RigidBody::Dynamic,
            VisibilityBundle { ..default() }, // Necessary to display child scene bundle
            ShootingTarget,
        ))
        .with_children(|child_builder| {
            // Colliders
            child_builder.spawn((
                TransformBundle::from(Transform::from_xyz(0., 2., 0.)),
                Collider::cuboid(0.2, 2., 0.2),
                ColliderMassProperties::Density(0.5),
            ));
            child_builder.spawn((
                Collider::cuboid(0.7, 0.3, 0.7),
                ColliderMassProperties::Density(2.),
            ));

            // Models
            child_builder.spawn(SceneBundle {
                scene: model_assets.scene_handles["raft_with_mast"].clone(),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            });
        })
        .id();

    // Spawn children that need a reference to the parent entity
    commands
        .entity(parent_entity)
        .with_children(|child_builder| {
            child_builder.spawn((
                SceneBundle {
                    scene: model_assets.scene_handles["pirate_flag"].clone(),
                    transform: Transform::from_xyz(0.0829, 3.2132, 0.0581),
                    ..default()
                },
                Pennant {
                    rig: Some(parent_entity),
                },
            ));
        });

    let pontoon_positions = [
        [-0.8, 0., 0.8],
        [0.8, 0., 0.8],
        [-0.8, 0., -0.8],
        [0.8, 0., -0.8],
        [0., 2., 0.],
    ];

    let pontoon_radius = 0.4;

    for pontoon_position in pontoon_positions {
        let position = Vec3::from_array(pontoon_position);
        let joint = FixedJointBuilder::new().local_anchor1(position);

        let child_pontoon = commands
            .spawn((
                TransformBundle::from(Transform::from_translation(spawn_at + position)),
                RigidBody::Dynamic,
                Collider::ball(pontoon_radius),
                ExternalForce { ..default() },
                Damping { ..default() },
                Velocity { ..default() },
                GravityScale(0.),
                CollisionGroups::new(Group::NONE, Group::NONE),
                Pontoon {
                    radius: pontoon_radius,
                    ..default()
                },
                ImpulseJoint::new(parent_entity, joint),
            ))
            .id();

        // Need to add pontoon to registry for later despawn
        shooting_target_despawn.entities.push(child_pontoon);
    }
}
