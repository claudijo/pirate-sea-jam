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
}

// Vec4 containing direction x, direction z, steepness, wave_length
// Sum of all steepness values must not exceed 1.
const first_wave = vec4<f32>(1., 0., 0.22, 36.);
const second_wave = vec4<f32>(1., 0.8, 0.2, 32.);
const third_wave = vec4<f32>(1., 1.2, 0.18, 28.);
const forth_wave = vec4<f32>(1., 3., 0.16, 24.);

struct OceanMaterial {
    quantize_steps: u32,
}

@group(1) @binding(100)
var<uniform> ocean_material: OceanMaterial;

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var grid_point = in.position;
    var tangent = vec3<f32>(1., 0., 0.);
    var binormal = vec3<f32>(0., 0., 1.);
    var p = grid_point;
    let time = globals.time;

    p += water_dynamics::gerstner_wave(first_wave, grid_point, &tangent, &binormal, time);
    p += water_dynamics::gerstner_wave(second_wave, grid_point, &tangent, &binormal, time);
    p += water_dynamics::gerstner_wave(third_wave, grid_point, &tangent, &binormal, time);
    p += water_dynamics::gerstner_wave(forth_wave, grid_point, &tangent, &binormal, time);

    var normal: vec3<f32> = normalize(cross(binormal, tangent));
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