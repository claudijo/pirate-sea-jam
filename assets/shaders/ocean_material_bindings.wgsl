#define_import_path pirate_sea_jam::ocean_material_bindings

const WAVES_COUNT: i32 = 4;

struct OceanTilelSettings {
    tile_offset: vec3<f32>,
    tile_size: f32,
    quad_cell_size: f32,
    tier: u32,
    time_scale: f32,
    waves: array<vec4<f32>, WAVES_COUNT>,
    subdivision_count: u32,
}

struct OceanPosition {
    center_offset: vec3<f32>,
}

struct RollbackTime {
    elapsed_seconds: f32,
    padding: vec3<f32>,
}

@group(1) @binding(100)
var<uniform> settings: OceanTilelSettings;

@group(1) @binding(101)
var<uniform> position: OceanPosition;

@group(1) @binding(102)
var<uniform> time: RollbackTime;