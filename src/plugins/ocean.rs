use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomOceanMaterial>>,
) {
    let mesh = Mesh::from(shape::Plane {
        size: 100.,
        subdivisions: 49,
    });

    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0., 0., 0.),
        material: materials.add(CustomOceanMaterial {}),
        ..default()
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomOceanMaterial {}

impl Material for CustomOceanMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_ocean_material.wgsl".into()
    }

}

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

    }
}
