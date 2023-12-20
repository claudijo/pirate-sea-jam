#define_import_path pirate_sea_jam::ocean_material_bindings

const WAVES_COUNT: i32 = 4;

struct OceanMaterial {
    grid_size: f32,
    tier: u32,
    offset: vec3<f32>,
    animation_time_scale: f32,
    waves: array<vec4<f32>, WAVES_COUNT>,
}

@group(1) @binding(100)
var<uniform> ocean_material: OceanMaterial;