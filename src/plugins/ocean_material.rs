use crate::resources::wave::WAVES;
use bevy::asset::load_internal_asset;
use bevy::math::Vec3A;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::reflect::TypeUuid;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroupShaderType, ShaderType};
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub const OCEAN_ANIMATION_TIME_SCALE: f32 = 0.6;

pub const OCEAN_TILE_SIZE: f32 = 40.;
const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 5; // Needs to be odd
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

pub type StandardOceanMaterial = ExtendedMaterial<StandardMaterial, OceanMaterial>;

// enum Tier {
//     Primary,
//     Secondary,
//     Tertiary,
// }

#[derive(Component)]
pub struct OceanTile;

fn spawn_ocean_tile(
    size: f32,
    subdivisions: u32,
    offset: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardOceanMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Plane { size, subdivisions });
    mesh.duplicate_vertices();

    // Use custom AABB to prevent culling issues of meshes when drawn after being animated and slightly displaced in the shader.
    const MAX_ANIMATED_VERTEX_DISPLACEMENT: f32 = 3.;
    let aabb = Aabb {
        center: Vec3A::ZERO,
        half_extents: Vec3A::new(
            size / 2. + MAX_ANIMATED_VERTEX_DISPLACEMENT,
            MAX_ANIMATED_VERTEX_DISPLACEMENT,
            size / 2. + MAX_ANIMATED_VERTEX_DISPLACEMENT,
        ),
    };

    let commands1 = commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_translation(offset),

            material: materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::rgb(0.15, 0.74, 0.86),
                    metallic: 1.,
                    ..Default::default()
                },
                extension: OceanMaterial {
                    grid_size: size / (subdivisions + 1) as f32,
                    offset,
                    animation_time_scale: OCEAN_ANIMATION_TIME_SCALE,
                    waves: WAVES,
                },
            }),
            ..default()
        },
        aabb,
        // AabbGizmo {
        //     color: Some(Color::PINK),
        // },
        OceanTile,
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, OceanMaterial>>>,
) {
    // Center tile
    spawn_ocean_tile(
        OCEAN_TILE_SIZE,
        OCEAN_PRIMARY_TILE_SUBDIVISIONS,
        Vec3::ZERO,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    for offset_base in OFFSET_BASES {
        // Secondary tiles
        spawn_ocean_tile(
            OCEAN_TILE_SIZE,
            OCEAN_SECONDARY_TILE_SUBDIVISIONS,
            offset_base * OCEAN_TILE_SIZE,
            &mut commands,
            &mut meshes,
            &mut materials,
        );

        // Tertiary tiles
        spawn_ocean_tile(
            OCEAN_TILE_SIZE * 3.,
            0,
            offset_base * OCEAN_TILE_SIZE * 3.,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[uuid = "20a0e61b-868f-4db2-b5c7-4bbc2b74b842"]
#[uniform(100, OceanMaterialUniform)]
pub struct OceanMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    grid_size: f32,
    offset: Vec3,
    animation_time_scale: f32,
    waves: [Vec4; 4],
}

#[derive(Clone, Default, ShaderType)]
pub struct OceanMaterialUniform {
    grid_size: f32,
    offset: Vec3,
    animation_time_scale: f32,
    waves: [Vec4; 4],
}

impl AsBindGroupShaderType<OceanMaterialUniform> for OceanMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> OceanMaterialUniform {
        OceanMaterialUniform {
            grid_size: self.grid_size,
            offset: self.offset,
            animation_time_scale: self.animation_time_scale,
            waves: self.waves,
        }
    }
}

impl MaterialExtension for OceanMaterial {
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

        app.add_plugins(MaterialPlugin::<StandardOceanMaterial> { ..default() });

        app.add_systems(Startup, setup);
    }
}
