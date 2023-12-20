#define_import_path pirate_sea_jam::utils

fn get_adjecent_grid_points(vertex_index: u32, grid_point: vec3<f32>, grid_size: f32) -> array<vec3<f32>,2>  {
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
        grid_point + cw_delta * grid_size,
        grid_point + ccw_delta * grid_size,
    );
}


fn smoothen_edges(vertex_index: u32, grid_point: vec3<f32>, tile_size: f32) -> vec3<f32> {


    return grid_point;
}
