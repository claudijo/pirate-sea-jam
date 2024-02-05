use crate::ocean::OCEAN_TILE_SIZE;
use crate::orbiting_camera::resources::OrbitingCamera;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    let pitch = 30_f32.to_radians();
    let radius = 30. + 15. * pitch;
    let translation = Vec3::new(0.0, pitch.sin() * radius, pitch.cos() * radius);

    commands.spawn((
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
    ));
}
