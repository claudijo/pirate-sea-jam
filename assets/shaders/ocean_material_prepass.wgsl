// Mainly copied from default StandardMaterial prepass shader and removed unused stuff
// See https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/prepass/prepass.wgsl
// Possibly merge with corresponding main shader.

#import bevy_pbr::{
    mesh_functions,
    prepass_io::{Vertex, VertexOutput},
    skinning,
    morph,
}

#import pirate_sea_jam::water_dynamics

// From https://github.com/rust-adventure/bevy-examples/blob/main/examples/dissolve-sphere-standard-material-extensions/assets/shaders/dissolve_material_prepass.wgsl
// Just importing `bevy_pbr::mesh_view_bindings::globals` will not work if running as prepass vertex shader
@group(0) @binding(1) var<uniform> globals: bevy_render::globals::Globals;

// Vec4 containing direction x, direction z, steepness, wave_length
// Sum of all steepness values must not exceed 1.
const first_wave = vec4<f32>(1., 0., 0.22, 36.);
const second_wave = vec4<f32>(1., 0.8, 0.2, 32.);
const third_wave = vec4<f32>(1., 1.2, 0.18, 28.);
const forth_wave = vec4<f32>(1., 3., 0.16, 24.);

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

    out.position = mesh_functions::mesh_position_local_to_clip(
        mesh_functions::get_model_matrix(in.instance_index),
        position
    );

    return out;
}