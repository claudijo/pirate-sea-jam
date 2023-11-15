use crate::game_state::GameState;
use crate::resources::wave_machine::WaveMachine;
use crate::systems::fluid_dynamics;
use bevy::prelude::*;

pub enum Tier {
    Primary,
    Secondary,
    Tertiary,
}

const OCEAN_SIZE: f32 = 240.;

const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 31; // Needs to be odd
const OCEAN_PRIMARY_TILE_SUBDIVISIONS: u32 = OCEAN_SECONDARY_TILE_SUBDIVISIONS * 2 + 1;


#[derive(Component)]
pub struct OceanTopology {
    pub mesh_positions: Vec<[f32; 3]>,
    pub size: f32,
    pub subdivisions: u32,
    pub tile_tier: Tier,
}

fn spawn_ocean_tile(
    size: f32,
    subdivisions: u32,
    translation: Vec3,
    tile_tier: Tier,
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
            tile_tier,
        },
    ));
}

fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Center tile
    spawn_ocean_tile(
        OCEAN_SIZE,
        OCEAN_PRIMARY_TILE_SUBDIVISIONS,
        Vec3::ZERO,
        Tier::Primary,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    for translation_base in [
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
            OCEAN_SIZE,
            OCEAN_SECONDARY_TILE_SUBDIVISIONS,
            translation_base * OCEAN_SIZE,
            Tier::Secondary,
            &mut commands,
            &mut meshes,
            &mut materials,
        );

        // Tertiary tiles
        spawn_ocean_tile(
            OCEAN_SIZE * 3.,
            0,
            translation_base * OCEAN_SIZE * 3.,
            Tier::Tertiary,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
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
