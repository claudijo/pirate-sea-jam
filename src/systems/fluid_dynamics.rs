use crate::components::pontoon::Pontoon;
use crate::plugins::ocean::{OceanTile, Tier};
use crate::resources::wave_machine::WaveMachine;
use crate::utils::tiles::{level_out, smoothen_edges};
use crate::utils::{liquid, liquid::SPHERE_DRAG_COEFFICIENT, sphere};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// https://stackoverflow.com/questions/72961896/how-do-i-modify-a-mesh-after-it-has-been-created-in-bevy-rust
pub fn make_waves(
    ocean_tile_query: Query<(&OceanTile, &Handle<Mesh>)>,
    mut assets: ResMut<Assets<Mesh>>,
    wave_machine: Res<WaveMachine>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();

    for (ocean_tile, handle) in &ocean_tile_query {
        if ocean_tile.was_culled {
            continue;
        }

        let mesh = assets.get_mut(handle).unwrap();
        let mut next_positions: Vec<[f32; 3]> = Vec::new();
        let mut next_colors: Vec<[f32; 4]> = Vec::new();

        for position in &ocean_tile.mesh_positions {
            let next_position =
                wave_machine.next_position(Vec3::from_array(*position), elapsed_time);

            next_positions.push(next_position.to_array());

            // This will be multiplied to the mesh base_color, assuming wave heights vary between
            // -2 and 2
            let color_multiplier = ((next_position[1] + 4.) / 8.).clamp(0., 1.);
            next_colors.push([color_multiplier, color_multiplier, color_multiplier, 1.])
        }

        let near = (ocean_tile.size.powf(2.) + ocean_tile.size.powf(2.)).sqrt() * 0.5;
        let far = ocean_tile.size * 1.5;

        let next_positions = match ocean_tile.tile_tier {
            Tier::Primary => smoothen_edges(next_positions, ocean_tile.subdivisions),
            Tier::Secondary => level_out(next_positions, &ocean_tile.mesh_positions, near, far),
            Tier::Tertiary => ocean_tile.mesh_positions.clone(),
        };

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, next_positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, next_colors);
        mesh.compute_flat_normals();
    }
}

pub fn buoyancy(
    mut pontoon_query: Query<(
        &Transform,
        &Pontoon,
        &Velocity,
        &mut ExternalForce,
        &mut Damping,
    )>,
    time: Res<Time>,
    wave_machine: Res<WaveMachine>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();
    for (transform, pontoon, velocity, mut external_force, mut damping) in &mut pontoon_query {
        let water_height = wave_machine.surface_height(transform.translation, elapsed_time);

        let displaced_liquid_volume =
            sphere::displaced_liquid_volume(pontoon.radius, transform.translation.y, water_height);

        let buoyant_force =
            liquid::buoyant_force(displaced_liquid_volume) * pontoon.buoyant_force_scale;

        let is_submerged = transform.translation.y - pontoon.radius < water_height;
        let linear_damping = if is_submerged {
            liquid::damping(
                velocity.linvel.y,
                sphere::cross_section_area(pontoon.radius),
                SPHERE_DRAG_COEFFICIENT,
            ) * pontoon.water_damping_scale
        } else {
            0.
        };

        external_force.force = buoyant_force;
        damping.linear_damping = linear_damping;
    }
}
