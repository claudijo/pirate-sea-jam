use crate::physics::components::{Buoy, Mass};
use bevy::prelude::*;

pub fn debug_buoys(buoy_query: Query<(&Buoy, &GlobalTransform)>, mut gizmos: Gizmos) {
    for (buoy, global_transform) in &buoy_query {
        gizmos.cuboid(
            Transform::from_matrix(global_transform.compute_matrix())
                .with_scale(Vec3::splat(buoy.max_depth * 2.)),
            Color::RED,
        )
    }
}

pub fn debug_physics_particle(
    particle_query: Query<(&Mass, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (mass, global_transform) in &particle_query {
        let (_, rotation, translation) = global_transform.to_scale_rotation_translation();
        gizmos.sphere(translation, rotation, mass.0 * 0.1, Color::PURPLE);
    }
}
