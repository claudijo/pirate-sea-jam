use bevy::prelude::*;
use std::f32::consts::PI;

pub const LIQUID_DENSITY: f32 = 1025.;
const GRAVITY: f32 = 9.807;
pub const SPHERE_DRAG_COEFFICIENT: f32 = 0.47;

// https://www.omnicalculator.com/physics/buoyancy
pub fn buoyant_force(displaced_liquid_volume: f32) -> Vec3 {
    Vec3::Y * LIQUID_DENSITY * displaced_liquid_volume * GRAVITY
}

// https://www.omnicalculator.com/physics/drag-equation
pub fn damping(relative_velocity: f32, reference_area: f32, drag_coefficient: f32) -> f32 {
    0.5 * LIQUID_DENSITY * relative_velocity.powi(2) * reference_area * drag_coefficient
}

// `wave`: Vec4 containing direction x, direction z, steepness, wave_length
pub fn gerstner_wave(wave: Vec4, p: Vec3, time: f32) -> Vec3 {
    let steepness = wave.z;
    let wave_length = wave.w;

    let k: f32 = 2. * PI / wave_length;
    let c: f32 = (GRAVITY / k).sqrt();
    let d: Vec2 = wave.xy().normalize();
    let f: f32 = k * (d.dot(p.xz()) - c * time);
    let a: f32 = steepness / k;

    Vec3::new(d.x * (a * f.cos()), a * f.sin(), d.y * (a * f.cos()))
}

// https://www.youtube.com/watch?v=kGEqaX4Y4bQ&t=746s
pub fn wave_height<F>(point: Vec3, time: f32, sample_count: u8, next_position: F) -> f32
where
    F: Fn(Vec3, f32) -> Vec3,
{
    let mut sample_point = point;
    let mut displacement;
    for _i in 1..sample_count {
        displacement = next_position(sample_point, time);
        sample_point -= displacement - point;
    }
    // Do last sample outside loop to avoid extra calculation
    displacement = next_position(sample_point, time);

    displacement.y
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
        let force = damping(5., 16., SPHERE_DRAG_COEFFICIENT);
        assert_eq!(force, 96350.0);
    }
}
