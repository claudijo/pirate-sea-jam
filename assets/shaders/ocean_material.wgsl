#import bevy_pbr::{
    mesh_view_bindings::globals,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world, mesh_normal_local_to_world},
}

#import bevy_render::instance_index::get_instance_index

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

struct OceanMaterial {
    world_offset: f32,
}

const pi: f32 = 3.1415926538;
const gravity: f32 = 9.807;

const steepness: f32 = 0.5;
const wave_length: f32 = 10.;
const direction: vec2<f32> = vec2<f32>(1., 1.);

@group(1) @binding(100)
var<uniform> ocean_material: OceanMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var position = vec4<f32>(vertex.position, 1.);

    let k: f32 = 2. * pi / wave_length;
    let c: f32 = sqrt(gravity / k);
    let a: f32 = steepness / k;
    let d: vec2<f32> = normalize(direction);
    let f = k * (dot(d, position.xz) - c * globals.time);

    position.x += d.x * (a * cos(f));
    position.y = a * sin(f);
    position.z += d.y * (a * cos(f));

    let tangent = vec3<f32>(
        1. - d.x * d.x * (steepness * sin(f)),
        d.x * (steepness * cos(f)),
        -d.x * d.y * (steepness * sin(f))
    );

    let binormal =vec3<f32>(
        -d.x * d.y * (steepness * sin(f)),
        d.y * (steepness * cos(f)),
        1. - d.y * d.y * (steepness * sin(f))
    );

    let normal = normalize(cross(binormal, tangent));

    out.position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance_index),
        position,
    );

    out.world_position = mesh_position_local_to_world(
        get_model_matrix(vertex.instance_index),
        position,
    );

    out.world_normal = mesh_normal_local_to_world(
        normal,
        vertex.instance_index
    );

    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else

    var out: FragmentOutput;

    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif


    return out;
}