use crate::components::ocean::OceanTopology;
use crate::game_state::GameState;
use crate::resources::wave_machine::WaveMachine;
use crate::systems::fluid_dynamics;
use bevy::prelude::*;

const OCEAN_SIZE: f32 = 50.;
const OCEAN_SUBDIVISIONS: u32 = 20;

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveMachine {
            time_scale: 0.4,
            sample_count: 4,
        })
        .add_systems(OnEnter(GameState::InGame), spawn_ocean)
        .add_systems(
            Update,
            (fluid_dynamics::make_waves).run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Plane {
        size: OCEAN_SIZE,
        subdivisions: OCEAN_SUBDIVISIONS,
    });

    // To facilitate `compute_flat_normals(` after manipulating mesh verticies
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
        Name::new("Ocean"),
    ));
}
