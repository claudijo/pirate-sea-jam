use bevy::prelude::*;

use crate::components::ocean::OceanTopology;
use crate::resources::wave_machine::WaveMachine;

// https://stackoverflow.com/q/72961896
pub fn make_waves(
    mut oceans: Query<(&OceanTopology, &Handle<Mesh>)>,
    mut assets: ResMut<Assets<Mesh>>,
    wave_machine: Res<WaveMachine>,
    time: Res<Time>,
) {
    let time = time.elapsed().as_secs_f32();
    for (ocean_topology, handle) in &mut oceans {
        let mesh = assets.get_mut(handle).unwrap();
        let mut next_positions: Vec<[f32; 3]> = Vec::new();
        let mut next_colors: Vec<[f32; 4]> = Vec::new();

        for position in &ocean_topology.positions {
            let next_position =
                wave_machine.next_position(Vec3::from_array(*position), time);

            next_positions.push(next_position.to_array());

            // This will be multiplied to the mesh base_color, assuming wave heights vary between 
            // -2 and 2
            let color_multiplier = ((next_position[1] + 4.) / 8.).clamp(0., 1.);
            next_colors.push([color_multiplier, color_multiplier, color_multiplier, 1.])
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, next_positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, next_colors);
        mesh.compute_flat_normals();
    }
}
