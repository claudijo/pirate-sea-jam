use crate::focal_point::resources::FocalPoint;
use crate::ocean::OCEAN_TILE_SIZE;
use crate::sky_box::components::Sky;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::pbr::{NotShadowCaster, PbrBundle, StandardMaterial};
use bevy::prelude::*;

pub fn spawn_sky_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sky box
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1., 1., 1.))),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("a5cddf").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(OCEAN_TILE_SIZE * 9.)),
            ..default()
        },
        Sky,
        NotShadowCaster,
    ));
}

pub fn sync_sky_box_center_offset(
    focal_point: Res<FocalPoint>,
    mut sky_box_query: Query<&mut Transform, With<Sky>>,
) {
    for mut transform in &mut sky_box_query {
        transform.translation = focal_point.0;
    }
}
