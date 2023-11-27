use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
    MaterialPipeline, MaterialPipelineKey, OpaqueRendererMethod,
};
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayout};
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, OceanMaterial>>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Plane {
        size: 10.,
        subdivisions: 99,
    });
    mesh.duplicate_vertices();

    // plane
    commands.spawn((MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0., 0., 0.),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::rgb(0.15, 0.74, 0.86),
                metallic: 1.,
                ..Default::default()
            },
            extension: OceanMaterial { world_offset: 10. },
        }),
        ..default()
    }));

    // Shadow cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: standard_materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 3., 0.0),
        ..default()
    });
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct OceanMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    world_offset: f32,
}

impl MaterialExtension for OceanMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }
}

pub struct OceanMaterialPlugin;

impl Plugin for OceanMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
