use bevy::asset::{embedded_asset, load_internal_asset};
use bevy::pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod};
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub const WATER_DYNAMICS_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x64632a74ee9240ea8097a33da35f3ad5);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, OceanMaterial>>>,
) {
    let mesh = Mesh::from(shape::Plane {
        size: 120.,
        subdivisions: 59,
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0., 0., 0.),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::rgb(0.15, 0.74, 0.86),
                metallic: 1.,
                ..Default::default()
            },
            extension: OceanMaterial { quantize_steps: 3 },
        }),
        ..default()
    });
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct OceanMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for OceanMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
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
