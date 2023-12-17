#define_import_path pirate_sea_jam::ocean_material_bindings

struct OceanMaterial {
    grid_size: f32,
    first_wave: vec4<f32>,
    second_wave: vec4<f32>,
    third_wave: vec4<f32>,
    fourth_wave: vec4<f32>,
}

@group(1) @binding(100)
var<uniform> ocean_material: OceanMaterial;