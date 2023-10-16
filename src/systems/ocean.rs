use crate::components::ocean::OceanTopology;
use bevy::prelude::*;

const OCEAN_SIZE: f32 = 50.;
const OCEAN_SUBDIVISIONS: u32 = 20;

pub fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Plane {
        size: OCEAN_SIZE,
        subdivisions: OCEAN_SUBDIVISIONS,
    });

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
                metallic: 0.7,
                perceptual_roughness: 0.3,
                ..default()
            }),
            ..default()
        },
        OceanTopology {
            positions: mesh_positions,
        },
    ));
}
