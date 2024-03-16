use bevy::prelude::*;
use crate::physics::components::Buoy;

pub fn debug_buoys(
    buoy_query: Query<(&Buoy, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (buoy, global_transform) in &buoy_query {
        gizmos.cuboid(
            Transform::from_matrix(global_transform.compute_matrix()).with_scale(Vec3::splat(buoy.max_depth*2.)),
            Color::RED,
        )
    }
}