use bevy::prelude::*;

pub const LIQUID_DENCITY: f32 = 1025.;
pub const GRAVITY: f32 = 9.81;
pub const CUBE_DRAG_COEFFICIENT: f32 = 1.05;

// https://www.omnicalculator.com/physics/buoyancy
pub fn buoyant_force(displaced_liquid_volume: f32) -> Vec3 {
    Vec3::Y * LIQUID_DENCITY * displaced_liquid_volume * GRAVITY
}

// https://www.omnicalculator.com/physics/drag-equation
pub fn damping(relative_velocity: f32, reference_area: f32, drag_coefficient: f32) -> f32 {
    0.5 * LIQUID_DENCITY * relative_velocity.powi(2) * reference_area * drag_coefficient
}

// http://www-evasion.imag.fr/Membres/Fabrice.Neyret/NaturalScenes/fluids/water/waves/fluids-nuages/waves/Jonathan/articlesCG/simulating-ocean-water-01.pdf
// wave_vector points in the direction of the travel of the wave
// For wave_vector.length() * amplitude > 1, an undesired loop forms at the tops of the wave
pub fn gerstner_wave(
    point_on_surface: Vec2,
    time: f32,
    wave_vector: Vec2,
    amplitude: f32,
    phase: f32,
) -> Vec3 {
    let wavenumber = wave_vector.length();
    let frequency = (GRAVITY * wavenumber).sqrt(); // Can be modified to take depth into account
    let xz = (wave_vector / wavenumber)
        * amplitude
        * (wave_vector.dot(point_on_surface) - frequency * time - phase).sin();
    let y = amplitude * (wave_vector.dot(point_on_surface) - frequency * time).cos();

    Vec3::new(xz.x, y, xz.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buoyancy() {
       let force = buoyant_force(64.);
       assert_eq!(force, Vec3::Y * 643536.);
    }

    #[test]
    fn drag() {
       let force = damping(5., 16., CUBE_DRAG_COEFFICIENT);
       assert_eq!(force, 215249.98);
    }
}

