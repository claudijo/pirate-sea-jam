#![allow(dead_code)]

use crate::components::ship::PlayerShip;
use crate::game_state::GameState;
use crate::resources::wave::Wave;
use crate::systems::fluid_dynamics;
use bevy::prelude::*;
use bevy::render::view::VisibilitySystems;
use crate::plugins::ocean_material::OCEAN_ANIMATION_TIME_SCALE;

pub enum Tier {
    Primary,
    Secondary,
    Tertiary,
}

pub const OCEAN_TILE_SIZE: f32 = 100.;

const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 19; // Needs to be odd
const OCEAN_PRIMARY_TILE_SUBDIVISIONS: u32 = OCEAN_SECONDARY_TILE_SUBDIVISIONS * 2 + 1;

#[derive(Component)]
pub struct OceanTile {
    pub mesh_positions: Vec<[f32; 3]>,
    pub size: f32,
    pub subdivisions: u32,
    pub tile_tier: Tier,
    pub was_culled: bool,
    pub offset: Vec3,
}

fn spawn_ocean_tile(
    size: f32,
    subdivisions: u32,
    tile_tier: Tier,
    offset: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Plane { size, subdivisions });

    // To facilitate `compute_flat_normals(` after manipulating mesh vertices
    mesh.duplicate_vertices();

    let mesh_positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .unwrap()
        .to_vec();

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.15, 0.74, 0.86),
                metallic: 1.,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_translation(offset),
            ..default()
        },
        OceanTile {
            mesh_positions,
            size,
            subdivisions,
            tile_tier,
            was_culled: false,
            offset,
        },
    ));
}

fn track_player_ship_position(
    ship_query: Query<&Transform, (With<PlayerShip>, Without<OceanTile>)>,
    mut ocean_tile_query: Query<(&mut Transform, &OceanTile)>,
) {
    for ship_transform in &ship_query {
        for (mut ocean_tile_transform, ocean_tile) in &mut ocean_tile_query {
            ocean_tile_transform.translation.x = ship_transform.translation.x + ocean_tile.offset.x;
            ocean_tile_transform.translation.z = ship_transform.translation.z + ocean_tile.offset.z;
        }
    }
}

fn check_ocean_tile_visibility(mut ocean_tile_query: Query<(&mut OceanTile, &ViewVisibility)>) {
    for (mut ocean_topology, view_visibility) in &mut ocean_tile_query {
        ocean_topology.was_culled = !view_visibility.get();
    }
}

fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Center tile
    spawn_ocean_tile(
        OCEAN_TILE_SIZE,
        OCEAN_PRIMARY_TILE_SUBDIVISIONS,
        Tier::Primary,
        Vec3::ZERO,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    for offset_base in [
        Vec3::new(0., 0., -1.),  // North
        Vec3::new(1., 0., -1.),  // North-east
        Vec3::new(1., 0., 0.),   // East
        Vec3::new(1., 0., 1.),   // South-east
        Vec3::new(0., 0., 1.),   // South
        Vec3::new(-1., 0., 1.),  // South-west
        Vec3::new(-1., 0., 0.),  // West
        Vec3::new(-1., 0., -1.), // North-west
    ] {
        // Secondary tiles
        spawn_ocean_tile(
            OCEAN_TILE_SIZE,
            OCEAN_SECONDARY_TILE_SUBDIVISIONS,
            Tier::Secondary,
            offset_base * OCEAN_TILE_SIZE,
            &mut commands,
            &mut meshes,
            &mut materials,
        );

        // Tertiary tiles
        spawn_ocean_tile(
            OCEAN_TILE_SIZE * 3.,
            0,
            Tier::Tertiary,
            offset_base * OCEAN_TILE_SIZE * 3.,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Wave {
            time_scale: OCEAN_ANIMATION_TIME_SCALE,
            sample_count: 4,
        });

        // app.add_systems(OnEnter(GameState::SplashScreen), spawn_ocean);

        app.add_systems(
            PostUpdate,
            check_ocean_tile_visibility.after(VisibilitySystems::CheckVisibility),
        );

        app.add_systems(
            Update,
            (
                fluid_dynamics::make_waves.run_if(in_state(GameState::SplashScreen)),
                fluid_dynamics::make_waves.run_if(in_state(GameState::InGame)),
            ),
        );

        app.add_systems(
            Update,
            (track_player_ship_position.run_if(in_state(GameState::InGame)),),
        );
    }
}
