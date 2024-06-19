use crate::ocean::components::OceanTile;
use crate::ocean::materials::{
    OceanMaterialExtension, OceanPosition, OceanTileSettings, RollbackTime, StandardOceanMaterial,
};
use crate::ocean::resources::{OceanCenter, Wave};
use crate::ocean::{
    OCEAN_PRIMARY_TILE_QUAD_CELL_SIZE, OCEAN_PRIMARY_TILE_SUBDIVISIONS,
    OCEAN_SECONDARY_TILE_SUBDIVISIONS, OCEAN_TILE_SIZE,
};
use crate::orbiting_camera::resources::FocalPoint;
use crate::physics::components::{AngularDrag, Buoy, LinearDrag};
use bevy::math::Vec3A;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_ggrs::Rollback;

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
    // https://github.com/bevyengine/bevy/pull/11773 and a comment on the finite/infinite plane split
    // PR (https://github.com/bevyengine/bevy/pull/12426/files#r1555080768) subdivision on the new
    // planes isn't implemented yet, so I'd continue using Plane instead of Plane3d if you need it,
    // even though its deprecated. Also, according to the tracking issue it is planned and not yet
    // done: https://github.com/bevyengine/bevy/issues/10572
    #[allow(deprecated)]
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
                    // AlphaMode required to calculate water depth i shaders (from the camera point
                    // of view).
                    alpha_mode: AlphaMode::Blend,
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
        NotShadowCaster,
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

// Nudge the ocean in the right direction every time the focal point (ship) traverses the distance
// equal to at least two times ocean tile quad size. (If just nudging one quad size, the secondary
// waves rendered at some far away distance will jump if visible. Assume adjustment might need be
// larger than two times ocean tile quad size, for example if the frame rate is low and/or tile quad
// size is small.
pub fn sync_ocean_global_center(
    mut ocean_center: ResMut<OceanCenter>,
    focal_point: Res<FocalPoint>,
) {
    let diff = focal_point.0 - ocean_center.0;

    let double_cell_size = OCEAN_PRIMARY_TILE_QUAD_CELL_SIZE * 2.;

    if diff.x.abs() > double_cell_size {
        ocean_center.0.x += (diff.x / double_cell_size).floor()
            * double_cell_size;
    }

    if diff.z.abs() > double_cell_size {
        ocean_center.0.z += (diff.z / double_cell_size).floor()
            * double_cell_size;
    }
}

pub fn sync_ocean_tiles_center_offset(
    ocean_center: Res<OceanCenter>,
    mut ocean_tile_query: Query<(&mut Transform, &OceanTile)>,
    mut materials: ResMut<Assets<StandardOceanMaterial>>,
) {
    for (mut transform, ocean_tile) in &mut ocean_tile_query {
        transform.translation = ocean_center.0 + ocean_tile.offset;
    }

    for (_, material) in materials.iter_mut() {
        material.extension.position.center_offset = ocean_center.0;
    }
}

pub fn sync_shader_time(time: Res<Time>, mut materials: ResMut<Assets<StandardOceanMaterial>>) {
    for (_, material) in materials.iter_mut() {
        material.extension.rollback_time.elapsed_seconds = time.elapsed_seconds();
    }
}

pub fn update_buoy_water_height(
    mut buoy_query: Query<(&GlobalTransform, &mut Buoy), With<Rollback>>,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_seconds();
    for (global_transform, mut buoy) in &mut buoy_query {
        buoy.water_height = wave.height(global_transform.translation(), wave.configs, elapsed_time);
    }
}

pub fn update_water_drag(
    mut ship_query: Query<(&GlobalTransform, &mut LinearDrag, &mut AngularDrag), With<Rollback>>,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_seconds();
    for (global_transform, mut linear_drag, mut angular_drag) in &mut ship_query {
        let water_height = wave.height(global_transform.translation(), wave.configs, elapsed_time);
        if global_transform.translation().y < water_height {
            linear_drag.velocity_drag_coefficient = 20.;
            linear_drag.velocity_squared_drag_coefficient = 30.;

            angular_drag.velocity_drag_coefficient = 20.;
            angular_drag.velocity_squared_drag_coefficient = 30.;
        } else {
            linear_drag.velocity_drag_coefficient = 0.;
            linear_drag.velocity_squared_drag_coefficient = 0.;

            angular_drag.velocity_drag_coefficient = 0.;
            angular_drag.velocity_squared_drag_coefficient = 0.;
        }
    }
}
