use crate::utils::water_mechanics;
use bevy::prelude::*;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Wave {
    pub time_scale: f32,
    pub sample_count: u8,
    pub configs: [Vec4; 4],
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct OceanCenter(pub Vec3);

impl Wave {
    pub fn next_position(&self, mut position: Vec3, waves: [Vec4; 4], time: f32) -> Vec3 {
        let time = time * self.time_scale;
        position.y = 0.; // Neutral water level

        position
            + waves
                .into_iter()
                .map(|wave| water_mechanics::gerstner_wave(wave, position, time))
                .sum::<Vec3>()
    }

    pub fn height(&self, point: Vec3, waves: [Vec4; 4], time: f32) -> f32 {
        water_mechanics::wave_height(
            point,
            waves,
            time,
            self.sample_count,
            |position: Vec3, waves: [Vec4; 4], time: f32| self.next_position(position, waves, time),
        )
    }
}
