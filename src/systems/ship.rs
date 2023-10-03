use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::pontoon::Pontoon;
use crate::components::ship::{Booster, Helm, Pennant, Sail, Ship, TurnRate};
use crate::resources::assets::ShipAssets;

pub fn spawn_ship(mut commands: Commands, ship_assets: Res<ShipAssets>) {
    let parent = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
            RigidBody::Dynamic,
            Collider::cuboid(0.8, 0.5, 2.),
            CollisionGroups::new(Group::NONE, Group::NONE),
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
        .id();

    let child_3d_model = commands
        .spawn(SceneBundle {
            scene: ship_assets.scene_handles["medium_hull"].clone(),
            transform: Transform::from_xyz(0., -0.5, 0.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Helm,
                SceneBundle {
                    scene: ship_assets.scene_handles["medium_helm"].clone(),
                    transform: Transform::from_xyz(0., 5.5806, -1.0694),
                    ..default()
                },
            ));
            parent.spawn((
                Sail,
                SceneBundle {
                    scene: ship_assets.scene_handles["medium_pirate_sail"].clone(),
                    transform: Transform::from_xyz(0., 2.3248, 1.3574),
                    ..default()
                },
            ));
            parent.spawn((
                Pennant,
                SceneBundle {
                    scene: ship_assets.scene_handles["medium_flag"].clone(),
                    transform: Transform::from_xyz(0., 9.38793, 1.35834),
                    ..default()
                },
            ));
            parent.spawn(SceneBundle {
                scene: ship_assets.scene_handles["port_back_canon"].clone(),
                transform: Transform::from_xyz(1.1769, 1.4593, -0.5485)
                    .with_rotation(Quat::from_rotation_z(PI)),
                ..default()
            });
            parent.spawn(SceneBundle {
                scene: ship_assets.scene_handles["port_front_canon"].clone(),
                transform: Transform::from_xyz(1.13846, 1.54822, 1.54781)
                    .with_rotation(Quat::from_rotation_z(PI)),
                ..default()
            });
            parent.spawn(SceneBundle {
                scene: ship_assets.scene_handles["starboard_back_canon"].clone(),
                transform: Transform::from_xyz(-1.1769, 1.4593, -0.5485),
                ..default()
            });
            parent.spawn(SceneBundle {
                scene: ship_assets.scene_handles["starboard_front_canon"].clone(),
                transform: Transform::from_xyz(-1.13846, 1.54822, 1.54781),
                ..default()
            });
        })
        .id();

    commands.entity(parent).push_children(&[child_3d_model]);

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

        let joint = FixedJointBuilder::new().local_anchor2(position);
        commands.entity(child_pontoon).with_children(|children| {
            children.spawn(ImpulseJoint::new(parent, joint));
        });
    }
}