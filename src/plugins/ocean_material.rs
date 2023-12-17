use bevy::asset::load_internal_asset;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroupShaderType, ShaderType};
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub const OCEAN_ANIMATION_TIME_SCALE: f32 = 0.6;

pub const OCEAN_TILE_SIZE: f32 = 200.;
const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 19; // Needs to be odd
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

pub const OCEAN_MATERIAL_BINDINGS: Handle<Shader> = Handle::weak_from_u128(0x06a957f34bac4aabad104c64a301c3fb);

pub type StandardOceanMaterial = ExtendedMaterial<StandardMaterial, OceanMaterial>;

enum Tier {
    Primary,
    Secondary,
    Tertiary,
}

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

    commands.spawn((
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
                    animation_time_scale: OCEAN_ANIMATION_TIME_SCALE,
                    first_wave: Vec4::new(1., 0., 0.22, 36.),
                    second_wave: Vec4::new(1., 0.8, 0.2, 32.),
                    third_wave: Vec4::new(1., 1.2, 0.18, 28.),
                    fourth_wave: Vec4::new(1., 3., 0.16, 24.),
                },
            }),
            ..default()
        },
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

    // for offset_base in &OFFSET_BASES {
    //     // Secondary tiles
    //     spawn_ocean_tile(
    //         OCEAN_TILE_SIZE,
    //         OCEAN_SECONDARY_TILE_SUBDIVISIONS,
    //         *offset_base * OCEAN_TILE_SIZE,
    //         &mut commands,
    //         &mut meshes,
    //         &mut materials,
    //     );
    //
    //     // Tertiary tiles
    //     spawn_ocean_tile(
    //         OCEAN_TILE_SIZE * 3.,
    //         0,
    //         *offset_base * OCEAN_TILE_SIZE * 3.,
    //         &mut commands,
    //         &mut meshes,
    //         &mut materials,
    //     );
    // }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[uuid = "8cdffa6c-7768-4405-b959-113767fdc3c5"]
#[uniform(100, OceanMaterialUniform)]
pub struct OceanMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    grid_size: f32,
    animation_time_scale: f32,
    first_wave: Vec4,
    second_wave: Vec4,
    third_wave: Vec4,
    fourth_wave: Vec4,
}

#[derive(Clone, Default, ShaderType)]
pub struct OceanMaterialUniform {
    grid_size: f32,
    animation_time_scale: f32,
    first_wave: Vec4,
    second_wave: Vec4,
    third_wave: Vec4,
    fourth_wave: Vec4,
}

impl AsBindGroupShaderType<OceanMaterialUniform> for OceanMaterial {
    fn as_bind_group_shader_type(&self, images: &RenderAssets<Image>) -> OceanMaterialUniform {
        OceanMaterialUniform {
            grid_size: self.grid_size,
            animation_time_scale: self.animation_time_scale,
            first_wave: self.first_wave,
            second_wave: self.second_wave,
            third_wave: self.third_wave,
            fourth_wave: self.fourth_wave,
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
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/ocean_material_bindings.wgsl"),
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<StandardOceanMaterial> {
            // prepass_enabled: false,
            ..default()
        });

        // app.register_asset_reflect::<OceanMaterial>()
        // .register_asset_reflect::<StandardOceanMaterial>();

        app.add_systems(Startup, setup);
    }
}
