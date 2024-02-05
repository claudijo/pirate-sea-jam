#define_import_path pirate_sea_jam::utils

#import pirate_sea_jam::{
    water_dynamics,
    ocean_material_bindings,
}

const VERTICES_PER_QUAD: u32 = 6u;

fn get_midpoint(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        (a[0] + b[0]) / 2.,
        (a[1] + b[1]) / 2.,
        (a[2] + b[2]) / 2.
    );
}

fn get_wave_adjusted_midpoint(a: vec3<f32>, b: vec3<f32>, time: f32) -> vec3<f32> {
    var grid_point_a = a;
    var grid_point_b = b;

    for (var i = 0; i < ocean_material_bindings::WAVES_COUNT; i += 1) {
        grid_point_a += water_dynamics::gerstner_wave(
            ocean_material_bindings::settings.waves[i],
            a + ocean_material_bindings::position.center_offset + ocean_material_bindings::settings.tile_offset,
            time
        );

        grid_point_b += water_dynamics::gerstner_wave(
            ocean_material_bindings::settings.waves[i],
            b + ocean_material_bindings::position.center_offset + ocean_material_bindings::settings.tile_offset,
            time
        );
    }

    return get_midpoint(grid_point_a, grid_point_b);
}

fn get_adjecent_grid_points(vertex_index: u32, grid_point: vec3<f32>, quad_cell_size: f32) -> array<vec3<f32>,2>  {
    var cw_delta: vec3<f32>;
    var ccw_delta: vec3<f32>;

    switch vertex_index % 6u {
        case 0u {
            cw_delta = vec3<f32>(-1., 0., 0.);
            ccw_delta = vec3<f32>(0., 0., -1.);
        }
        case 1u {
            cw_delta = vec3<f32>(0., 0., 1.);
            ccw_delta = vec3<f32>(-1., 0., 1.);
        }
        case 2u {
            cw_delta = vec3<f32>(1., 0., -1.);
            ccw_delta = vec3<f32>(1., 0., 0.);
        }
        case 3u {
            cw_delta = vec3<f32>(1., 0., 0.);
            ccw_delta = vec3<f32>(0., 0., 1.);
        }
        case 4u {
            cw_delta = vec3<f32>(0., 0., -1.);
            ccw_delta = vec3<f32>(1., 0., -1.);
        }
        case 5u {
            cw_delta = vec3<f32>(-1., 0., 1.);
            ccw_delta = vec3<f32>(-1., 0., 0.);
        }
        default {
            cw_delta = vec3<f32>(0.);
            ccw_delta = vec3<f32>(0.);
        }
    }

    return array(
        grid_point + cw_delta * quad_cell_size,
        grid_point + ccw_delta * quad_cell_size,
    );
}

fn get_grid_point_north(grid_point: vec3<f32>, quad_cell_size: f32) -> vec3<f32> {
    return grid_point + vec3<f32>(0., 0., -quad_cell_size);
}

fn get_grid_point_south(grid_point: vec3<f32>, quad_cell_size: f32) -> vec3<f32> {
    return grid_point + vec3<f32>(0., 0., quad_cell_size);
}

fn get_grid_point_east(grid_point: vec3<f32>, quad_cell_size: f32) -> vec3<f32> {
    return grid_point + vec3<f32>(quad_cell_size, 0., 0.);
}

fn get_grid_point_west(grid_point: vec3<f32>, quad_cell_size: f32) -> vec3<f32> {
    return grid_point + vec3<f32>(-quad_cell_size, 0., 0.);
}

fn level_out(
    next_position: vec3<f32>,
    initial_position: vec3<f32>,
    offset: vec3<f32>,
    near: f32,
    far: f32,
) -> vec3<f32> {
    let span = far - near;
    let distance = length(initial_position + offset);
    let clamped = clamp(distance, near, far);
    let scale = 1. - (clamped - near) / span;

    return mix(initial_position, next_position, scale);
}

fn smoothen_edges(
    vertex_index: u32,
    position: vec3<f32>,
    subdivision_count: u32,
    quad_cell_size: f32,
    default_position: vec3<f32>,
    time: f32
) -> vec3<f32> {
    let tile_size = subdivision_count + 1u;
    let row = vertex_index / (tile_size * VERTICES_PER_QUAD);
    let col = (vertex_index / VERTICES_PER_QUAD) % tile_size;

    if row == 0u { // First row
        let order = vertex_index % VERTICES_PER_QUAD;

        if col % 2u == 0u { // Even col
            if order == 1u || order == 5u {
                return get_wave_adjusted_midpoint(
                    get_grid_point_west(position, quad_cell_size),
                    get_grid_point_east(position, quad_cell_size),
                    time
                );
            }
        } else { // Odd col
            if order == 3u {
                 return get_wave_adjusted_midpoint(
                    get_grid_point_west(position, quad_cell_size),
                    get_grid_point_east(position, quad_cell_size),
                    time
                 );
            }
        }
    } else if row == subdivision_count { // Last row
        let order = vertex_index % VERTICES_PER_QUAD;

        if col % 2u == 0u { // Even col
            if order == 0u {
                return get_wave_adjusted_midpoint(
                    get_grid_point_west(position, quad_cell_size),
                    get_grid_point_east(position, quad_cell_size),
                    time
                );
            }
        } else { // Odd col
            if order == 2u || order == 4u {
                return get_wave_adjusted_midpoint(
                    get_grid_point_west(position, quad_cell_size),
                    get_grid_point_east(position, quad_cell_size),
                    time
                );
            }
        }
    }

    if col == 0u { // First col
        let order = vertex_index % VERTICES_PER_QUAD;
        if row % 2u == 0u { // Even row
            if order == 2u || order == 4u {
                return get_wave_adjusted_midpoint(
                    get_grid_point_north(position, quad_cell_size),
                    get_grid_point_south(position, quad_cell_size),
                    time
                );
            }
        } else { // Odd row
            if order == 3u {
                return get_wave_adjusted_midpoint(
                    get_grid_point_north(position, quad_cell_size),
                    get_grid_point_south(position, quad_cell_size),
                    time
                );
            }
        }
    } else if col == subdivision_count { // Last col
        let order = vertex_index % VERTICES_PER_QUAD;

        if row % 2u == 0u { // Even row
            if order == 0u {
                return get_wave_adjusted_midpoint(
                    get_grid_point_north(position, quad_cell_size),
                    get_grid_point_south(position, quad_cell_size),
                    time
                );
            }
        } else { // Odd row
            if order == 1u || order == 5u {
                return get_wave_adjusted_midpoint(
                    get_grid_point_north(position, quad_cell_size),
                    get_grid_point_south(position, quad_cell_size),
                    time
                );
            }
        }
    }

    return default_position;
}
