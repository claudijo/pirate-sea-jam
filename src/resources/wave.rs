use crate::utils::water_dynamics;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Wave {
    pub time_scale: f32,
    pub sample_count: u8,
}

// Each Vec4 containing direction x, direction z, steepness, wave_length
// Sum of all steepness values must not exceed 1.
pub const WAVES: [Vec4; 4] = [
    Vec4::new(1., 0., 0.22, 36.),
    Vec4::new(1., 0.8, 0.2, 32.),
    Vec4::new(1., 1.2, 0.18, 28.),
    Vec4::new(1., 3., 0.16, 24.),
];

impl Wave {
    pub fn next_position(&self, mut position: Vec3, time: f32) -> Vec3 {
        position.y = 0.; // Neutral water level

        let time = time * self.time_scale;

        position + WAVES.into_iter()
            .map(|wave| water_dynamics::gerstner_wave(wave, position, time))
            .sum::<Vec3>()
    }

    pub fn surface_height(&self, point: Vec3, time: f32) -> f32 {
        water_dynamics::wave_height(
            point,
            time,
            self.sample_count,
            |position: Vec3, time: f32| self.next_position(position, time),
        )
    }
}
