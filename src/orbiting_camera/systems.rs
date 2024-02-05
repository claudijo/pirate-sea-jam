use crate::focal_point::resources::FocalPoint;
use crate::orbiting_camera::events::OrbitMotion;
use crate::orbiting_camera::resources::OrbitingCamera;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn orbit(
    window_query: Query<&Window>,
    mut orbit_event_reader: EventReader<OrbitMotion>,
    mut orbiting_camera_query: Query<(&mut Transform, &mut OrbitingCamera)>,
    focal_point: Res<FocalPoint>,
) {
    let mut orbit_move = Vec2::ZERO;
    for orbit_event in orbit_event_reader.read() {
        orbit_move += orbit_event.delta;
    }

    for (mut camera_transform, mut orbiting_camera) in &mut orbiting_camera_query {
        if orbit_move.length_squared() > 0.0 {
            let window = window_query.single();
            let window_width = window.resolution.width();
            let window_height = window.resolution.height();

            let delta_x = orbit_move.x / window_width * PI * 2.0;
            let delta_y = orbit_move.y / window_height * PI;

            orbiting_camera.pitch = (orbiting_camera.pitch + delta_y)
                .clamp(orbiting_camera.min_pitch, orbiting_camera.max_pitch);
            orbiting_camera.yaw += delta_x;

            orbiting_camera.radius = 30. + 15. * orbiting_camera.pitch;

            let yaw = Quat::from_rotation_y(-orbiting_camera.yaw);
            let pitch = Quat::from_rotation_x(-orbiting_camera.pitch);

            camera_transform.rotation = yaw * pitch;
        }

        // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
        // parent = x and y rotation
        // child = z-offset
        let rot_matrix = Mat3::from_quat(camera_transform.rotation);
        camera_transform.translation =
            focal_point.0 + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbiting_camera.radius));
    }
}
