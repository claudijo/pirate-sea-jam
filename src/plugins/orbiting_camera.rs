// From https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

use crate::components::ship::PlayerShip;
use crate::game_state::GameState;
use bevy::prelude::*;
use bevy_rapier3d::parry::math::DEFAULT_EPSILON;
use std::f32::consts::PI;

#[derive(Component)]
pub struct OrbitingCamera {
    pub central_position: Vec3,
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
}

impl Default for OrbitingCamera {
    fn default() -> Self {
        OrbitingCamera {
            central_position: Vec3::ZERO,
            radius: 10.,
            pitch: 30_f32.to_radians(),
            yaw: 0.,
            min_pitch: 20_f32.to_radians(),
            max_pitch: PI / 2.,
        }
    }
}

#[derive(Event)]
pub struct OrbitEvent {
    pub delta: Vec2,
}

fn orbit(
    window_query: Query<&Window>,
    mut orbit_event_reader: EventReader<OrbitEvent>,
    mut orbiting_camera_query: Query<(&mut Transform, &mut OrbitingCamera)>,
) {
    let mut orbit_move = Vec2::ZERO;
    for orbit_event in &mut orbit_event_reader {
        orbit_move += orbit_event.delta;
    }

    for (mut transform, mut orbiting_camera) in &mut orbiting_camera_query {
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

            transform.rotation = yaw * pitch;
        }

        // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
        // parent = x and y rotation
        // child = z-offset
        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation = orbiting_camera.central_position
            + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbiting_camera.radius));
    }
}

fn center(
    mut orbit_camera_query: Query<&mut OrbitingCamera>,
    ship_query: Query<&Transform, With<PlayerShip>>,
) {
    if let Ok(transform) = ship_query.get_single() {
        if let Ok(mut orbit_camera) = orbit_camera_query.get_single_mut() {
            orbit_camera.central_position = transform.translation;
        }
    }
}

pub struct OrbitingCameraPlugin;

impl Plugin for OrbitingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrbitEvent>();

        app.add_systems(Update, (orbit, center).run_if(in_state(GameState::InGame)));
    }
}
