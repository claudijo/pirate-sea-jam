use crate::components::pontoon::{CubePontoonSize, PontoonForceScale};
use crate::game_state::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_ship);
    }
}

fn spawn_ship(mut commands: Commands) {
    let parent = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(0., 5., 0.)),
            RigidBody::Dynamic,
            Collider::cuboid(2.5, 0.5, 1.),
            CollisionGroups::new(Group::NONE, Group::NONE),
        ))
        .id();

    for row in 0..3 {
        for col in 0..2 {
            let position = Vec3::new(-2.5 + row as f32 * 2.5, 0., -1. + col as f32 * 2.);

            let child = commands
                .spawn((
                    TransformBundle::from(Transform::from_translation(position)),
                    RigidBody::Dynamic,
                    Collider::ball(0.5),
                    ExternalForce { ..default() },
                    Damping { ..default() },
                    Velocity { ..default() },
                    CollisionGroups::new(Group::NONE, Group::NONE),
                    CubePontoonSize { side: 1. },
                    PontoonForceScale {
                        buoyant_force_scale: 0.005,
                        water_damping_scale: 0.0005,
                    },
                ))
                .id();

            let joint = FixedJointBuilder::new().local_anchor2(position);
            commands.entity(child).with_children(|children| {
                children.spawn(ImpulseJoint::new(parent, joint));
            });
        }
    }
}
