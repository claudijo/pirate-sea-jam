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
    let time = globals.time * ocean_material_bindings::settings.animation_time_scale;

    var out: VertexOutput;
    var next_position = in.position;

    for (var i = 0; i < ocean_material_bindings::WAVES_COUNT; i += 1) {
        next_position += water_dynamics::gerstner_wave(
            ocean_material_bindings::settings.waves[i],
            in.position + ocean_material_bindings::settings.offset,
            time
        );
    }

    var position = vec4<f32>(next_position, 1.);

    out.position = mesh_functions::mesh_position_local_to_clip(
        mesh_functions::get_model_matrix(in.instance_index),
        position
    );

    return out;
}