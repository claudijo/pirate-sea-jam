#import bevy_pbr::{
    mesh_view_bindings::globals,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    forward_io::{FragmentOutput, VertexOutput, Vertex},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world, mesh_normal_local_to_world},
}

//struct Vertex {
//    @builtin(instance_index) instance_index: u32,
//#ifdef VERTEX_POSITIONS
//    @location(0) position: vec3<f32>,
//#endif
//#ifdef VERTEX_NORMALS
//    @location(1) normal: vec3<f32>,
//#endif
//#ifdef VERTEX_UVS
//    @location(2) uv: vec2<f32>,
//#endif
//// (Alternate UVs are at location 3, but they're currently unused here.)
//#ifdef VERTEX_TANGENTS
//    @location(4) tangent: vec4<f32>,
//#endif
//#ifdef VERTEX_COLORS
//    @location(5) color: vec4<f32>,
//#endif
//#ifdef SKINNED
//    @location(6) joint_indices: vec4<u32>,
//    @location(7) joint_weights: vec4<f32>,
//#endif
//#ifdef MORPH_TARGETS
//    @builtin(vertex_index) index: u32,
//#endif
//};

//struct VertexOutput {
//    // This is `clip position` when the struct is used as a vertex stage output
//    // and `frag coord` when used as a fragment stage input
//    @builtin(position) position: vec4<f32>,
//    @location(0) world_position: vec4<f32>,
//    @location(1) world_normal: vec3<f32>,
//#ifdef VERTEX_UVS
//    @location(2) uv: vec2<f32>,
//#endif
//#ifdef VERTEX_TANGENTS
//    @location(3) world_tangent: vec4<f32>,
//#endif
//#ifdef VERTEX_COLORS
//    @location(4) color: vec4<f32>,
//#endif
//#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
//    @location(5) @interpolate(flat) instance_index: u32,
//#endif
//}

//struct FragmentOutput {
//    @location(0) color: vec4<f32>,
//}

const pi: f32 = 3.1415926538;
const gravity: f32 = 9.807;

fn gerstner_wave(
    wave: vec4<f32>,
    p: vec3<f32>,
    tangent: ptr<function,vec3<f32>>,
    binormal: ptr<function,vec3<f32>>,
) -> vec3<f32> {
    let steepness = wave.z;
    let wave_length = wave.w;

   let k: f32 = 2. * pi / wave_length;
   let c: f32 = sqrt(gravity / k);
   let d: vec2<f32> = normalize(wave.xy);
   let f: f32 = k * (dot(d, p.xz) - c * globals.time);
   let a: f32 = steepness / k;

    *tangent += vec3<f32>(
        -d.x * d.x * (steepness * sin(f)),
        d.x * (steepness * cos(f)),
        -d.x * d.y * (steepness * sin(f))
    );

    *binormal += vec3<f32>(
        -d.x * d.y * (steepness * sin(f)),
        d.y * (steepness * cos(f)),
        -d.y * d.y * (steepness * sin(f))
    );

    return vec3<f32>(
        d.x * (a * cos(f)),
        a * sin(f),
        d.y * (a * cos(f))
    );
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

    p += gerstner_wave(first_wave, grid_point, &tangent, &binormal);
    p += gerstner_wave(second_wave, grid_point, &tangent, &binormal);
    p += gerstner_wave(third_wave, grid_point, &tangent, &binormal);
    p += gerstner_wave(forth_wave, grid_point, &tangent, &binormal);

    var normal: vec3<f32> = normalize(cross(binormal, tangent));
    var position = vec4<f32>(p, 1.);

    out.position = mesh_position_local_to_clip(
        get_model_matrix(in.instance_index),
        position,
    );

    out.world_position = mesh_position_local_to_world(
        get_model_matrix(in.instance_index),
        position,
    );

    out.world_normal = mesh_normal_local_to_world(
        normal,
        in.instance_index
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