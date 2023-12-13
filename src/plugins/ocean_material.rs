use bevy::asset::{embedded_asset, load_internal_asset};
use bevy::pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod};
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub const OCEAN_TILE_SIZE: f32 = 100.;
const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 19; // Needs to be odd
const OCEAN_PRIMARY_TILE_SUBDIVISIONS: u32 = OCEAN_SECONDARY_TILE_SUBDIVISIONS * 2 + 1;

pub const WATER_DYNAMICS_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x64632a74ee9240ea8097a33da35f3ad5);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, OceanMaterial>>>,
) {
    let mut mesh = Mesh::from(shape::Plane {
        size: OCEAN_TILE_SIZE,
        subdivisions: OCEAN_SECONDARY_TILE_SUBDIVISIONS,
    });

    mesh.duplicate_vertices();

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0., 0., 0.),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::rgb(0.15, 0.74, 0.86),
                metallic: 1.,
                ..Default::default()
            },
            extension: OceanMaterial {
                grid_size: OCEAN_TILE_SIZE / (OCEAN_PRIMARY_TILE_SUBDIVISIONS + 1) as f32
            },
        }),
        ..default()
    });
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct OceanMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    grid_size: f32,
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

        app.add_systems(Startup, setup);
    }
}
