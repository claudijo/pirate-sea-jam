use bevy::prelude::*;

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
    let frequency = (9.81 * wavenumber).sqrt(); // Can be modified to take depth into account
    let xz = (wave_vector / wavenumber)
        * amplitude
        * (wave_vector.dot(point_on_surface) - frequency * time - phase).sin();
    let y = amplitude * (wave_vector.dot(point_on_surface) - frequency * time).cos();

    Vec3::new(xz.x, y, xz.y)
}
