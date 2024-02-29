use crate::utils::water_mechanics;
use bevy::prelude::*;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Wave {
    pub time_scale: f32,
    pub sample_count: u8,
    pub configs: [Vec4; 4],
}

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

    pub fn next_position_normal(
        &self,
        mut position: Vec3,
        waves: [Vec4; 4],
        time: f32,
    ) -> (Vec3, Vec3) {
        let time = time * self.time_scale;
        position.y = 0.; // Neutral water level

        let mut tangent = Vec3::new(1., 0., 0.);
        let mut binormal = Vec3::new(0., 0., 1.);

        let position = position
            + waves
                .into_iter()
                .map(|wave| {
                    water_mechanics::gerstner_wave_tangent_binormal(
                        wave,
                        position,
                        &mut tangent,
                        &mut binormal,
                        time,
                    )
                })
                .sum::<Vec3>();

        let normal = (binormal.cross(tangent)).normalize();

        (position, normal)
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
