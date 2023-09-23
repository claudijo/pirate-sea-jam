use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub turn_rate: f32,
    pub speed_factor: f32,
    pub maneuverability: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            turn_rate: 0.,
            speed_factor: 0.4,
            maneuverability: 0.1,
        }
    }
}

#[derive(Component)]
pub struct Helm;
