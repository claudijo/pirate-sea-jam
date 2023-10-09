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
    let physics_parent = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(20., 0., 20.)),
            RigidBody::Dynamic,
            VisibilityBundle { ..default() }, // Necessary to display child scene bundle
            ShootingTarget,
        ))
        .with_children(|parent| {
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(0., 2., 0.)),
                Collider::cuboid(0.2, 2., 0.2),
                ColliderMassProperties::Density(0.),
            ));
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
                Collider::cuboid(0.7, 0.3, 0.7),
            ));
        })
        .id();

    let child_3d_models = commands
        .spawn(SceneBundle {
            scene: model_assets.scene_handles["raft_with_mast"].clone(),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Pennant {
                    rig: Some(physics_parent),
                },
                SceneBundle {
                    scene: model_assets.scene_handles["pirate_flag"].clone(),
                    transform: Transform::from_xyz(0.0829, 3.2132, 0.0581),
                    ..default()
                },
            ));
        })
        .id();

    commands
        .entity(physics_parent)
        .push_children(&[child_3d_models]);

    let pontoon_positions = [
        [-0.7, 0., 0.7],
        [0.7, 0., 0.7],
        [-0.7, 0., -0.7],
        [0.7, 0., -0.7],
    ];

    let pontoon_radius = 0.3;

    for pontoon_position in pontoon_positions {
        let position = Vec3::from_array(pontoon_position);
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
            ))
            .id();

        // Need to add pontoon to registry for later despawn
        shooting_target_despawn.entities.push(child_pontoon);

        let joint = FixedJointBuilder::new().local_anchor2(position);
        commands.entity(child_pontoon).with_children(|children| {
            let joint_entity = children
                .spawn(ImpulseJoint::new(physics_parent, joint))
                .id();

            // Need to add joint to registry for later despawn
            shooting_target_despawn.entities.push(joint_entity);
        });
    }
}
