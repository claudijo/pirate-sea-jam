#import bevy_pbr::{
    mesh_view_bindings::globals,
    pbr_fragment,
    forward_io::{FragmentOutput, VertexOutput, Vertex},
    pbr_functions,
    mesh_functions,
}

#import bevy_render::instance_index

#import pirate_sea_jam::{
    water_dynamics,
    utils,
    ocean_material_bindings,
}

@vertex
fn vertex(in: Vertex, @builtin(vertex_index) vertex_index : u32) -> VertexOutput {
    var out: VertexOutput;

    let time = globals.time * ocean_material_bindings::ocean_material.animation_time_scale;

    var p = in.position;

    let adjecent_grid_points: array<vec3<f32>,2> = utils::get_adjecent_grid_points(
        vertex_index,
        in.position,
        ocean_material_bindings::ocean_material.grid_size
    );

    let grid_point_cw = adjecent_grid_points[0];
    let grid_point_ccw = adjecent_grid_points[1];

    var p_cw = grid_point_cw;
    var p_ccw = grid_point_ccw;

    for (var i = 0; i < ocean_material_bindings::WAVES_COUNT; i += 1) {
        p += water_dynamics::gerstner_wave(
            ocean_material_bindings::ocean_material.waves[i],
            in.position + ocean_material_bindings::ocean_material.offset,
            time
        );
        p_cw += water_dynamics::gerstner_wave(
            ocean_material_bindings::ocean_material.waves[i],
            grid_point_cw + ocean_material_bindings::ocean_material.offset,
            time
        );
        p_ccw += water_dynamics::gerstner_wave(
            ocean_material_bindings::ocean_material.waves[i],
            grid_point_ccw + ocean_material_bindings::ocean_material.offset,
            time
        );
    }

    var normal: vec3<f32> = normalize(cross(p_ccw - p, p_cw - p));
    var position = vec4<f32>(p, 1.);
    var model = mesh_functions::get_model_matrix(in.instance_index);

    out.position = mesh_functions::mesh_position_local_to_clip(
        model,
        position,
    );

    out.world_position = mesh_functions::mesh_position_local_to_world(
        model,
        position,
    );

    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        normal,
        instance_index::get_instance_index(in.instance_index)
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
    var pbr_input = pbr_fragment::pbr_input_from_standard_material(in, is_front);

    // apply lighting
    out.color = pbr_functions::apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = pbr_functions::main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}