use crate::game_state::GameState;
use crate::resources::wave_machine::WaveMachine;
use crate::systems::fluid_dynamics;
use bevy::prelude::*;

pub enum TileTier {
    Primary, Secondary,
    Tertiary
}

const OCEAN_SIZE: f32 = 300.;
const OCEAN_PRIMARY_TILE_SUBDIVISIONS: u32 = 79;
const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 39;

#[derive(Component)]
pub struct OceanTopology {
    pub mesh_positions: Vec<[f32; 3]>,
    pub size: f32,
    pub subdivisions: u32,
    pub tile_order: TileTier,
}

fn spawn_ocean_tile(
    size: f32,
    subdivisions: u32,
    translation: Vec3,
    tile_order: TileTier,
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
        .into_iter()
        .map(|pos| {
            [
                pos[0] + translation.x,
                pos[1] + translation.y,
                pos[2] + translation.z,
            ]
        })
        .collect();

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.15, 0.74, 0.86),
                metallic: 0.7,
                perceptual_roughness: 0.3,
                ..default()
            }),
            ..default()
        },
        OceanTopology {
            mesh_positions,
            size,
            subdivisions,
            tile_order,
        },
    ));
}

fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_ocean_tile(
        OCEAN_SIZE,
        OCEAN_PRIMARY_TILE_SUBDIVISIONS,
        Vec3::ZERO,
        TileTier::Primary,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_ocean_tile(
        OCEAN_SIZE,
        OCEAN_SECONDARY_TILE_SUBDIVISIONS,
        Vec3::new(0., 0., -OCEAN_SIZE),
        TileTier::Secondary,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    //
    // spawn_ocean_tile(
    //     OCEAN_SIZE,
    //     OCEAN_SECONDARY_TILE_SUBDIVISIONS,
    //     Vec3::new(-OCEAN_SIZE, 0., -OCEAN_SIZE),
    //     TileOrder::Secondary,
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    // );
    //
    // spawn_ocean_tile(
    //     OCEAN_SIZE,
    //     OCEAN_SECONDARY_TILE_SUBDIVISIONS,
    //     Vec3::new(-OCEAN_SIZE, 0., 0.),
    //     TileOrder::Secondary,
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    // );
    //
    // spawn_ocean_tile(
    //     OCEAN_SIZE,
    //     OCEAN_SECONDARY_TILE_SUBDIVISIONS,
    //     Vec3::new(OCEAN_SIZE, 0., 0.),
    //     TileOrder::Secondary,
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    // );
    //
    // spawn_ocean_tile(
    //     OCEAN_SIZE,
    //     OCEAN_SECONDARY_TILE_SUBDIVISIONS,
    //     Vec3::new(OCEAN_SIZE, 0., -OCEAN_SIZE),
    //     TileOrder::Secondary,
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    // );
    //
    spawn_ocean_tile(
        OCEAN_SIZE * 3.,
        0,
        Vec3::new(0., 0., -3. * OCEAN_SIZE),
        TileTier::Tertiary,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveMachine {
            time_scale: 0.4,
            sample_count: 4,
        })
        .add_systems(OnEnter(GameState::SplashScreen), spawn_ocean)
        .add_systems(
            Update,
            (
                fluid_dynamics::make_waves.run_if(in_state(GameState::SplashScreen)),
                fluid_dynamics::make_waves.run_if(in_state(GameState::InGame)),
            ),
        );
    }
}
