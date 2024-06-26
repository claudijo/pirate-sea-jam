#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    forward_io::{FragmentOutput, VertexOutput, Vertex},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world, mesh_normal_local_to_world},
}

//#import bevy_render::instance_index

#import pirate_sea_jam::{
    water_dynamics,
    utils,
    ocean_material_bindings,
}

// Note:`in.position` does not seems to include tranlsation done when creating the material. Associated translation etc.
// seems to be be applied using the provided mesh function
@vertex
fn vertex(in: Vertex, @builtin(vertex_index) vertex_index : u32) -> VertexOutput {
    let time = ocean_material_bindings::time.elapsed_seconds * ocean_material_bindings::settings.time_scale;

    var out: VertexOutput;
    var next_position = in.position;

    let adjecent_grid_points: array<vec3<f32>,2> = utils::get_adjecent_grid_points(
        vertex_index,
        in.position,
        ocean_material_bindings::settings.quad_cell_size
    );

    let position_cw = adjecent_grid_points[0];
    let position_ccw = adjecent_grid_points[1];

    var next_position_cw = position_cw;
    var next_position_ccw = position_ccw;

    for (var i = 0; i < ocean_material_bindings::WAVES_COUNT; i += 1) {
        next_position += water_dynamics::gerstner_wave(
            ocean_material_bindings::settings.waves[i],
            in.position + ocean_material_bindings::position.center_offset + ocean_material_bindings::settings.tile_offset,
            time
        );
        next_position_cw += water_dynamics::gerstner_wave(
            ocean_material_bindings::settings.waves[i],
            position_cw + ocean_material_bindings::position.center_offset + ocean_material_bindings::settings.tile_offset,
            time
        );
        next_position_ccw += water_dynamics::gerstner_wave(
            ocean_material_bindings::settings.waves[i],
            position_ccw + ocean_material_bindings::position.center_offset + ocean_material_bindings::settings.tile_offset,
            time
        );
    }

    switch ocean_material_bindings::settings.tier {
        case 0u: {
            next_position = utils::smoothen_edges(
                vertex_index,
                in.position,
                ocean_material_bindings::settings.subdivision_count,
                ocean_material_bindings::settings.quad_cell_size,
                next_position,
                time
            );
        }
        case 1u: {
            let tile_size_cubed = pow(ocean_material_bindings::settings.tile_size, 2.);
            let near = sqrt(tile_size_cubed * 2.) * 0.5;
            let far = ocean_material_bindings::settings.tile_size * 1.5;
            next_position = utils::level_out(
                next_position,
                in.position,
                ocean_material_bindings::settings.tile_offset,
                near,
                far
            );
        }
        default { // case 2u
            next_position = in.position;
        }
    }

    var normal: vec3<f32> = normalize(cross(next_position_ccw - next_position, next_position_cw - next_position));
    var position = vec4<f32>(next_position, 1.);
    var model_matrix = get_model_matrix(in.instance_index);

    out.position = mesh_position_local_to_clip(
        model_matrix,
        position,
    );

    out.world_position = mesh_position_local_to_world(
        model_matrix,
        position,
    );

    out.world_normal = mesh_normal_local_to_world(
        normal,
        in.instance_index
//        instance_index::get_instance_index(in.instance_index)
    );

    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var out: FragmentOutput;

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}