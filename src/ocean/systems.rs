use crate::focal_point::resources::FocalPoint;
use crate::ocean::components::OceanTile;
use crate::ocean::materials::{
    OceanMaterialExtension, OceanPosition, OceanTileSettings, RollbackTime, StandardOceanMaterial,
};
use crate::ocean::resources::Wave;
use crate::ocean::{
    OCEAN_PRIMARY_TILE_SUBDIVISIONS, OCEAN_SECONDARY_TILE_SUBDIVISIONS, OCEAN_TILE_SIZE,
};
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

#[derive(Clone, Copy)]
pub enum Tier {
    Primary,
    Secondary,
    Tertiary,
}

const OFFSET_BASES: [Vec3; 8] = [
    Vec3::new(0., 0., -1.),  // North
    Vec3::new(1., 0., -1.),  // North-east
    Vec3::new(1., 0., 0.),   // East
    Vec3::new(1., 0., 1.),   // South-east
    Vec3::new(0., 0., 1.),   // South
    Vec3::new(-1., 0., 1.),  // South-west
    Vec3::new(-1., 0., 0.),  // West
    Vec3::new(-1., 0., -1.), // North-west
];

pub fn spawn_ocean_tile(
    tile_size: f32,
    subdivision_count: u32,
    waves: [Vec4; 4],
    time_scale: f32,
    offset: Vec3,
    tier: Tier,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardOceanMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Plane {
        size: tile_size,
        subdivisions: subdivision_count,
    });
    mesh.duplicate_vertices();

    // Use custom AABB to prevent culling issues of meshes after being animated and displaced in the shader.
    const MAX_ANIMATED_VERTEX_DISPLACEMENT: f32 = 3.6;
    let aabb = Aabb {
        center: Vec3A::ZERO,
        half_extents: Vec3A::new(
            tile_size / 2. + MAX_ANIMATED_VERTEX_DISPLACEMENT,
            MAX_ANIMATED_VERTEX_DISPLACEMENT,
            tile_size / 2. + MAX_ANIMATED_VERTEX_DISPLACEMENT,
        ),
    };

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_translation(offset),
            material: materials.add(StandardOceanMaterial {
                base: StandardMaterial {
                    base_color: Color::rgb(0.15, 0.74, 0.86),
                    metallic: 1.,
                    ..Default::default()
                },
                extension: OceanMaterialExtension {
                    settings: OceanTileSettings {
                        tile_offset: offset,
                        tile_size,
                        quad_cell_size: tile_size / (subdivision_count + 1) as f32,
                        tier: tier as u32,
                        time_scale,
                        waves,
                        subdivision_count,
                    },
                    position: OceanPosition {
                        center_offset: Vec3::ZERO,
                    },
                    rollback_time: RollbackTime::default(),
                },
            }),
            ..default()
        },
        aabb,
        OceanTile { offset },
        Name::new("Ocean tile"),
    ));
}

pub fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardOceanMaterial>>,
    wave: Res<Wave>,
) {
    // Center tile
    spawn_ocean_tile(
        OCEAN_TILE_SIZE,
        OCEAN_PRIMARY_TILE_SUBDIVISIONS,
        wave.configs,
        wave.time_scale,
        Vec3::ZERO,
        Tier::Primary,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    for offset_base in OFFSET_BASES {
        // Secondary tiles
        spawn_ocean_tile(
            OCEAN_TILE_SIZE,
            OCEAN_SECONDARY_TILE_SUBDIVISIONS,
            wave.configs,
            wave.time_scale,
            offset_base * OCEAN_TILE_SIZE,
            Tier::Secondary,
            &mut commands,
            &mut meshes,
            &mut materials,
        );

        // Tertiary tiles
        spawn_ocean_tile(
            OCEAN_TILE_SIZE * 3.,
            0,
            wave.configs,
            wave.time_scale,
            offset_base * OCEAN_TILE_SIZE * 3.,
            Tier::Tertiary,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

pub fn sync_ocean_tiles_center_offset(
    focal_point: Res<FocalPoint>,
    mut ocean_tile_query: Query<(&mut Transform, &OceanTile)>,
    mut materials: ResMut<Assets<StandardOceanMaterial>>,
) {
    for (mut transform, ocean_tile) in &mut ocean_tile_query {
        transform.translation = focal_point.0 + ocean_tile.offset;
    }

    for (_, material) in materials.iter_mut() {
        material.extension.position.center_offset = focal_point.0;
    }
}

pub fn sync_shader_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardOceanMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.extension.rollback_time.elapsed_seconds = time.elapsed_seconds();
    }
}
