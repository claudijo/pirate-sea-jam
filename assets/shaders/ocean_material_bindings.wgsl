#define_import_path pirate_sea_jam::ocean_material_bindings

const WAVES_COUNT: i32 = 4;

// Keep in sync with rust code
const IMPACT_POINTS_CAPACITY: i32 = 40;

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
}

struct ImpactPoint {
    elapsed_seconds: f32,
    position: vec3<f32>,
}

struct ImpactPoints {
    write_index: u32,
    read_index: u32,
    buffer: array<ImpactPoint, IMPACT_POINTS_CAPACITY>,
}

@group(2) @binding(100)
var<uniform> settings: OceanTilelSettings;

@group(2) @binding(101)
var<uniform> position: OceanPosition;

@group(2) @binding(102)
var<uniform> time: RollbackTime;

@group(2) @binding(103)
var<uniform> impact_points: ImpactPoints;