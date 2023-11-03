use std::f32::consts::PI;
use std::slice::Windows;
use bevy::prelude::*;
use bevy::window::{Cursor, CursorGrabMode};
use crate::components::ship::{PlayerShip, Ship};
use crate::events::camera::CameraControllerEvent;


pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}

pub fn follow_ship(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Ship>)>,
    ship_query: Query<&Transform, With<Ship>>
) {


    for mut camera_transform in &mut camera_query {
        for ship_transform in &ship_query {
            // camera_transform.translation = Vec3::new(0.0, 20.0, 40.0) + ship_transform.translation;



        }
    }
}

// TODO: Don't use this
fn get_primary_window_size(window: &Query<&Window>) -> Vec2 {
    let window = window.single();
    let window = Vec2::new(window.resolution.width() as f32, window.resolution.height() as f32);
    window
}

pub fn orbit(
    window: Query<&Window>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerShip>)>,
    ship_query: Query<&Transform, With<PlayerShip>>,
    mut camera_controller_event_reader: EventReader<CameraControllerEvent>,
) {

        for mut camera_transform in &mut camera_query {
            for ship_transform in &ship_query {
                for event in camera_controller_event_reader.iter() {
                    let mut rotation_move = Vec2::ZERO;
                    rotation_move += event.movement_delta * 0.2;

                    if rotation_move.length_squared() > 0.0 {
                        let window = get_primary_window_size(&window);
                        let delta_x = rotation_move.x / window.x * PI * 2.0;
                        let delta_y = rotation_move.y / window.y * PI;

                        let yaw = Quat::from_rotation_y(-delta_x);
                        let pitch = Quat::from_rotation_x(-delta_y);

                        camera_transform.rotation = yaw * camera_transform.rotation; // rotate around global y axis
                        camera_transform.rotation = camera_transform.rotation * pitch; // rotate around local x axis
                    }
                }

                // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
                // parent = x and y rotation
                // child = z-offset
                let rot_matrix = Mat3::from_quat(camera_transform.rotation);
                camera_transform.translation = ship_transform.translation + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, 30.));
            }

    }
}

pub fn grab_pointer(
    mut window: Query<&mut Window>
) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor = Cursor {
            icon: Default::default(),
            visible: false,
            grab_mode: CursorGrabMode::Locked,
            hit_test: true,
        };
    }
}

pub fn release_pointer (
    mut window: Query<&mut Window>
) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor = Cursor::default();
    }
}

pub fn release_pointer_on_escape (
    mut window: Query<&mut Window>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = window.get_single_mut() {
            window.cursor = Cursor::default();
        }
    }
}