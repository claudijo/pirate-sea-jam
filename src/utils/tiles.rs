use bevy::prelude::*;

const VERTICES_PER_QUAD: usize = 6;

fn get_midpoint(a: [f32;3], b: [f32;3]) -> [f32;3] {
    [(a[0] + b[0]) / 2., (a[1] + b[1]) / 2., (a[2] + b[2]) / 2.]
}

pub fn smoothen_edges(mut positions: Vec<[f32; 3]>, subdivisions_count: u32) -> Vec<[f32; 3]> {
    let subdivisions_count = subdivisions_count as usize;
    let size = subdivisions_count + 1;


    for i in 0..positions.len() {
        let row = i / (size * VERTICES_PER_QUAD);
        let col = (i / VERTICES_PER_QUAD) % size;

        if row == 0 { // First row
            let order = i % VERTICES_PER_QUAD;

            if col % 2 == 0 { // Even col
                if order == 1 {
                    let midpoint = get_midpoint(positions[i + 2], positions[i + 10]);
                    positions[i] = midpoint;
                }
                if order == 5 {
                    let midpoint = get_midpoint(positions[i -2], positions[i + 6]);
                    positions[i] = midpoint;
                }
            } else { // Odd col
                if order == 3 {
                    let midpoint = get_midpoint(positions[i - 6], positions[i + 2]);
                    positions[i] = midpoint;
                }
            }
        } else if row == subdivisions_count { // Last row
            let order = i % VERTICES_PER_QUAD;

            if col % 2 == 0 { // Even col
                if order == 0 {
                    let midpoint = get_midpoint(positions[i + 2], positions[i + 6]);
                    positions[i] = midpoint;
                }
            } else { // Odd col
                if order == 2 {
                    let midpoint = get_midpoint(positions[i - 6], positions[i - 2]);
                    positions[i] = midpoint;
                }
                if order == 4 {
                    let midpoint = get_midpoint(positions[i - 6], positions[i - 4]);
                    positions[i] = midpoint;
                }
            }
        }

        if col == 0 { // First col
            let order = i % VERTICES_PER_QUAD;

            if row % 2 == 0 { // Even row
                if order == 2 {
                    let midpoint = get_midpoint(positions[i + 1], positions[i + size * VERTICES_PER_QUAD]);
                    positions[i] = midpoint;
                }

                if order == 4 {
                    let midpoint = get_midpoint(positions[i - 1], positions[i + size * VERTICES_PER_QUAD]);
                    positions[i] = midpoint;
                }
            } else { // Odd row
                let order = i % VERTICES_PER_QUAD;

                if order == 3 {
                    let midpoint = get_midpoint(positions[i - size * VERTICES_PER_QUAD], positions[i + 1]);
                    positions[i] = midpoint;
                }
            }
        } else if col == subdivisions_count { // Last col
            let order = i % VERTICES_PER_QUAD;

            if row % 2 == 0 { // Even row
                if order == 0 {
                    let midpoint = get_midpoint(positions[i + 1], positions[i + size * VERTICES_PER_QUAD]);
                    positions[i] = midpoint;
                }

            } else { // Odd row
                if order == 1 {
                    let midpoint = get_midpoint(positions[i - size * VERTICES_PER_QUAD], positions[i - 1]);
                    positions[i] = midpoint;
                }

                if order == 5 {
                    let midpoint = get_midpoint(positions[i - size * VERTICES_PER_QUAD], positions[i - 5]);
                    positions[i] = midpoint;
                }
            }
        }
    }

    positions
}
