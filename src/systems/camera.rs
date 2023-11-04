use crate::plugins::orbiting_camera::OrbitingCamera;
use bevy::prelude::*;
use bevy::window::{Cursor, CursorGrabMode};

pub fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(0.0, 20.0, 40.0);
    let radius = translation.length();
    let target = Vec3::ZERO;

    commands.spawn((
        OrbitingCamera {
            radius,
            central_position: target,
        },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 40.0).looking_at(target, Vec3::Y),
            ..default()
        },
    ));
}

pub fn grab_pointer(mut window: Query<&mut Window>) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor = Cursor {
            icon: Default::default(),
            visible: false,
            grab_mode: CursorGrabMode::Locked,
            hit_test: true,
        };
    }
}

pub fn release_pointer(mut window: Query<&mut Window>) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor = Cursor::default();
    }
}

pub fn release_pointer_on_escape(window: Query<&mut Window>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Escape) {
        release_pointer(window);
    }
}
