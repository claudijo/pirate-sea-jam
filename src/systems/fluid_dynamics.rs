use crate::components::ocean::OceanTopology;
use crate::components::pontoon::{CubePontoonSize, PontoonForceScale};
use crate::resources::wave_machine::WaveMachine;
use crate::utils::{cube, liquid, liquid::CUBE_DRAG_COEFFICIENT};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// https://stackoverflow.com/questions/72961896/how-do-i-modify-a-mesh-after-it-has-been-created-in-bevy-rust
pub fn make_waves(
    mut oceans: Query<(&OceanTopology, &Handle<Mesh>)>,
    mut assets: ResMut<Assets<Mesh>>,
    wave_machine: Res<WaveMachine>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();
    for (ocean_topology, handle) in &mut oceans {
        let mesh = assets.get_mut(handle).unwrap();
        let mut next_positions: Vec<[f32; 3]> = Vec::new();
        let mut next_colors: Vec<[f32; 4]> = Vec::new();

        for position in &ocean_topology.positions {
            let next_position =
                wave_machine.next_position(Vec3::from_array(*position), elapsed_time);

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

pub fn buoyancy(
    mut pontoons: Query<(
        &Transform,
        &CubePontoonSize,
        &Velocity,
        &PontoonForceScale,
        &mut ExternalForce,
        &mut Damping,
    )>,
    time: Res<Time>,
    wave_machine: Res<WaveMachine>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();
    for (transform, pontoon_size, velocity, pontoon_force_scale, mut external_force, mut damping) in
        &mut pontoons
    {
        let water_height = wave_machine.surface_height(transform.translation, elapsed_time);

        let displaced_liquid_volume =
            cube::displaced_liquid_volume(pontoon_size.side, transform.translation.y, water_height);

        let buoyant_force = liquid::buoyant_force(displaced_liquid_volume)
            * pontoon_force_scale.buoyant_force_scale;

        let is_submerged = transform.translation.y - pontoon_size.side / 2. < water_height;
        let linear_damping = if is_submerged {
            liquid::damping(
                velocity.linvel.y,
                pontoon_size.side.powi(2),
                CUBE_DRAG_COEFFICIENT,
            ) * pontoon_force_scale.water_damping_scale
        } else {
            0.
        };

        external_force.force = buoyant_force;
        damping.linear_damping = linear_damping;
    }
}
