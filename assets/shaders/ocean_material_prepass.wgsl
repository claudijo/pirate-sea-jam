// Mainly copied from default StandardMaterial prepass shader and removed unused stuff
// See https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/prepass/prepass.wgsl
// Possibly merge with corresponding main shader.

#import bevy_pbr::{
    mesh_functions,
    prepass_io::{Vertex, VertexOutput},
    skinning,
    morph,
}

#import pirate_sea_jam::{
    water_dynamics,
    ocean_material_bindings,
}

// From https://github.com/rust-adventure/bevy-examples/blob/main/examples/dissolve-sphere-standard-material-extensions/assets/shaders/dissolve_material_prepass.wgsl
// Just importing `bevy_pbr::mesh_view_bindings::globals` will not work if running as prepass vertex shader
@group(0) @binding(1) var<uniform> globals: bevy_render::globals::Globals;

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    let time = globals.time * ocean_material_bindings::ocean_material.animation_time_scale;
    var out: VertexOutput;
    var p = in.position;

    for (var i = 0; i < ocean_material_bindings::WAVES_COUNT; i += 1) {
        p += water_dynamics::gerstner_wave(ocean_material_bindings::ocean_material.waves[i], in.position, time);
    }

    var position = vec4<f32>(p, 1.);

    out.position = mesh_functions::mesh_position_local_to_clip(
        mesh_functions::get_model_matrix(in.instance_index),
        position
    );

    return out;
}