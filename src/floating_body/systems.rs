use crate::floating_body::components::{FloatingPosition, Yaw};
use crate::ocean::resources::Wave;
use crate::utils::vec2_extensions::Vec2Ext;
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

pub fn float(
    mut floating_body_query: Query<(&mut Transform, &Yaw, &FloatingPosition), With<Rollback>>,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_seconds();
    for (mut transform, yaw, position) in &mut floating_body_query {
        let (next_position, normal) =
            wave.next_position_normal(position.0.extend_with_y(0.), wave.configs, elapsed_time);

        transform.translation = next_position;
        transform.rotation =
            Quat::from_axis_angle(normal, yaw.0) * Quat::from_rotation_arc(Vec3::Y, normal);
    }
}
