use crate::components::ship::PlayerShip;
use crate::game_state::GameState;
use crate::resources::wave::{NO_WAVES, WAVES};
use bevy::asset::load_internal_asset;
use bevy::math::Vec3A;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::render::primitives::Aabb;
use bevy::render::render_resource::ShaderType;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub const OCEAN_ANIMATION_TIME_SCALE: f32 = 0.6;
pub const OCEAN_TILE_SIZE: f32 = 100.;
const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 15; // Needs to be odd
const OCEAN_PRIMARY_TILE_SUBDIVISIONS: u32 = OCEAN_SECONDARY_TILE_SUBDIVISIONS * 2 + 1;

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

pub const WATER_DYNAMICS_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x64632a74ee9240ea8097a33da35f3ad5);

pub const UTILS_HANDLE: Handle<Shader> = Handle::weak_from_u128(0x24c6df2a389f4396aa11f2840f30c5da);

pub const OCEAN_MATERIAL_BINDINGS: Handle<Shader> =
    Handle::weak_from_u128(0x06a957f34bac4aabad104c64a301c3fb);

pub type StandardOceanMaterial = ExtendedMaterial<StandardMaterial, OceanMaterialExtension>;

#[derive(Component)]
pub struct OceanTile {
    pub offset: Vec3,
}

#[derive(Clone, Copy)]
pub enum Tier {
    Primary,
    Secondary,
    Tertiary,
}

fn spawn_ocean_tile(
    tile_size: f32,
    subdivision_count: u32,
    waves: [Vec4; 4],
    position: Vec3,
    tier: Tier,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardOceanMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Plane { size: tile_size, subdivisions: subdivision_count });
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
            transform: Transform::from_translation(position),
            material: materials.add(StandardOceanMaterial {
                base: StandardMaterial {
                    base_color: Color::rgb(0.15, 0.74, 0.86),
                    metallic: 1.,
                    ..Default::default()
                },
                extension: OceanMaterialExtension {
                    settings: OceanTileSettings {
                        tile_size,
                        quad_cell_size: tile_size / (subdivision_count + 1) as f32,
                        tier: tier as u32,
                        offset: position,
                        animation_time_scale: OCEAN_ANIMATION_TIME_SCALE,
                        waves,
                        subdivision_count,
                    },
                },
            }),
            ..default()
        },
        aabb,
        AabbGizmo {
            color: Some(Color::PINK),
        },
        OceanTile { offset: position },
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardOceanMaterial>>,
) {
    // Center tile
    spawn_ocean_tile(
        OCEAN_TILE_SIZE,
        OCEAN_PRIMARY_TILE_SUBDIVISIONS,
        WAVES,
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
            WAVES,
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
            NO_WAVES,
            offset_base * OCEAN_TILE_SIZE * 3.,
            Tier::Tertiary,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

#[derive(ShaderType, Clone, Reflect, Debug)]
struct OceanTileSettings {
    tile_size: f32,
    quad_cell_size: f32,
    tier: u32,
    offset: Vec3,
    animation_time_scale: f32,
    waves: [Vec4; 4],
    subdivision_count: u32,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct OceanMaterialExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    settings: OceanTileSettings,
}

impl MaterialExtension for OceanMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/ocean_material_prepass.wgsl".into()
    }
}

fn track_player_ship_position(
    ship_query: Query<&Transform, (With<PlayerShip>, Without<OceanTile>)>,
    mut ocean_tile_query: Query<(&mut Transform, &mut OceanTile)>,
) {
    for ship_transform in &ship_query {
        for (mut ocean_tile_transform, ocean_tile) in &mut ocean_tile_query {
            ocean_tile_transform.translation.x = ship_transform.translation.x + ocean_tile.offset.x;
            ocean_tile_transform.translation.z = ship_transform.translation.z + ocean_tile.offset.z;
        }
    }
}

pub struct OceanMaterialPlugin;

impl Plugin for OceanMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            WATER_DYNAMICS_HANDLE,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/water_dynamics.wgsl"
            ),
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            UTILS_HANDLE,
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/utils.wgsl"),
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            OCEAN_MATERIAL_BINDINGS,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/ocean_material_bindings.wgsl"
            ),
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, OceanMaterialExtension>,
        >::default());

        app.add_systems(Startup, setup);

        app.add_systems(
            Update,
            track_player_ship_position.run_if(in_state(GameState::InGame)),
        );
    }
}
