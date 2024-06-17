// Shader that positions ocean verices so that shadows are calculated correctly. Quick and dirty, meaninging that only
// most grannular waves closest to the ship will be fully correct.
// Mainly copied from default StandardMaterial prepass shader and removed unused stuff
// See https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/prepass/prepass.wgsl
// Possibly merge with corresponding main shader. Been tried using `#ifdef PREPASS_PIPELINE` directive without success.

#import bevy_pbr::{
    mesh_functions::{mesh_position_local_to_clip, get_model_matrix},
    prepass_io::{Vertex, VertexOutput},
}

#import pirate_sea_jam::{
    water_dynamics::gerstner_wave,
    ocean_material_bindings,
}

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    let time = ocean_material_bindings::time.elapsed_seconds * ocean_material_bindings::settings.time_scale;

    var out: VertexOutput;
    var next_position = in.position;

    for (var i = 0; i < ocean_material_bindings::WAVES_COUNT; i += 1) {
        next_position += gerstner_wave(
            ocean_material_bindings::settings.waves[i],
            in.position + ocean_material_bindings::position.center_offset + ocean_material_bindings::settings.tile_offset,
            time
        );
    }

    var position = vec4<f32>(next_position, 1.);

    out.position = mesh_position_local_to_clip(
        get_model_matrix(in.instance_index),
        position
    );

    return out;
}