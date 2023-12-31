use crate::utils::liquid;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Resource)]
pub struct WaveMachine {
    pub time_scale: f32,
    pub sample_count: u8,
}

impl WaveMachine {
    pub fn next_position(&self, position: Vec3, time: f32) -> Vec3 {
        let point_on_surface = Vec2::new(position.x, position.z);
        let time = time * self.time_scale;
        let mut offset_sum = Vec3::ZERO;

        // Quick and dirty waves. Should introduce concepts like the speed and direction of the wind
        // and the fetch (the distance over which the wind blows) to determine the waves' size and
        // shape. Could also use a configurable value for number of modulations.
        let phase = 0.;

        offset_sum += liquid::gerstner_wave(
            point_on_surface,
            time,
            Vec2::from_angle(0.) * 0.2,
            1.2,
            phase,
        );

        offset_sum += liquid::gerstner_wave(
            point_on_surface,
            time,
            Vec2::from_angle(PI / 6.) * 0.4,
            0.6,
            phase,
        );

        offset_sum += liquid::gerstner_wave(
            point_on_surface,
            time,
            Vec2::from_angle(PI / 4.) * 0.6,
            0.4,
            phase,
        );

        offset_sum += liquid::gerstner_wave(
            point_on_surface,
            time,
            Vec2::from_angle(PI / 2.) * 0.8,
            0.2,
            phase,
        );

        Vec3::new(
            position.x - offset_sum.x,
            offset_sum.y,
            position.z - offset_sum.z,
        )
    }

    pub fn surface_height(&self, point: Vec3, time: f32) -> f32 {
        liquid::wave_height(
            point,
            time,
            self.sample_count,
            |position: Vec3, time: f32| self.next_position(position, time),
        )
    }
}
