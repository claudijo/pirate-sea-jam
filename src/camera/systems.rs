use crate::camera::resources::MainCamera;
use crate::ocean::OCEAN_TILE_SIZE;
use crate::orbiting_camera::components::OrbitingCamera;
use bevy::prelude::*;
use bevy::window::{Cursor, CursorGrabMode};

pub fn spawn_camera(mut commands: Commands) {
    let pitch = 30_f32.to_radians();
    let radius = 30. + 15. * pitch;
    let translation = Vec3::new(0.0, pitch.sin() * radius, pitch.cos() * radius);

    let main_camera_id = commands
        .spawn((
            OrbitingCamera {
                pitch,
                radius,
                ..default()
            },
            Camera3dBundle {
                transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            FogSettings {
                color: Color::hex("a5cddf").unwrap(),
                directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
                directional_light_exponent: 30.0,
                falloff: FogFalloff::Linear {
                    start: OCEAN_TILE_SIZE * 0.25,
                    end: OCEAN_TILE_SIZE * 1.5,
                },
            },
        ))
        .id();

    commands.insert_resource(MainCamera { id: main_camera_id });
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

pub fn release_pointer_on_escape(window: Query<&mut Window>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Escape) {
        release_pointer(window);
    }
}
