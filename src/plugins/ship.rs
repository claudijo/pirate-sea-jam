use crate::components::pontoon::{CubePontoonSize, PontoonForceScale};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship);
    }
}

fn spawn_ship(mut commands: Commands) {
    let position = Vec3::new(0., 10., 0.);

    commands
        .spawn((
            TransformBundle::from(Transform::from_translation(position)),
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            ExternalForce { ..default() },
            Damping { ..default() },
            Velocity { ..default() },
            CollisionGroups::new(Group::NONE, Group::NONE),
            CubePontoonSize { side: 1. },
            PontoonForceScale {
                buoyant_force_scale: 0.002,
                linear_damping_scale: 0.0002,
            },
        ));
}
