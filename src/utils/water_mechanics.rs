use bevy::prelude::*;
use std::f32::consts::PI;

const GRAVITY: f32 = 10.;

// See: https://catlikecoding.com/unity/tutorials/flow/waves/
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

pub fn gerstner_wave_tangent_binormal(
    wave: Vec4,
    p: Vec3,
    tangent: &mut Vec3,
    binormal: &mut Vec3,
    time: f32,
) -> Vec3 {
    let steepness = wave.z;
    let wave_length = wave.w;

    let k: f32 = 2. * PI / wave_length;
    let c: f32 = (GRAVITY / k).sqrt();
    let d: Vec2 = wave.xy().normalize();
    let f: f32 = k * (d.dot(p.xz()) - c * time);
    let a: f32 = steepness / k;

    *tangent += Vec3::new(
        -d.x * d.x * (steepness * f.sin()),
        d.x * (steepness * f.cos()),
        -d.x * d.y * (steepness * f.sin()),
    );

    *binormal += Vec3::new(
        -d.x * d.y * (steepness * f.sin()),
        d.y * (steepness * f.cos()),
        -d.y * d.y * (steepness * f.sin()),
    );

    Vec3::new(d.x * (a * f.cos()), a * f.sin(), d.y * (a * f.cos()))
}

// https://www.youtube.com/watch?v=kGEqaX4Y4bQ&t=746s
pub fn wave_height<F>(
    point: Vec3,
    waves: [Vec4; 4],
    time: f32,
    sample_count: u8,
    next_position: F,
) -> f32
where
    F: Fn(Vec3, [Vec4; 4], f32) -> Vec3,
{
    let mut sample_point = point;
    let mut displacement;
    for _i in 1..sample_count {
        displacement = next_position(sample_point, waves, time);
        sample_point -= displacement - point;
    }
    // Do last sample outside loop to avoid extra calculation
    displacement = next_position(sample_point, waves, time);

    displacement.y
}
