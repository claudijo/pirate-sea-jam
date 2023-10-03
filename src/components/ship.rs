use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub booster_power: f32,
    pub maneuverability: f32,
    pub stability: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            speed: 3.,
            maneuverability: 0.8,
            booster_power: 160.,
            stability: 4.,
        }
    }
}

#[derive(Component)]
pub struct TurnRate {
    pub value: f32,
}

impl Default for TurnRate {
    fn default() -> Self {
        Self { value: 0. }
    }
}

#[derive(Component)]
pub struct Booster {
    pub active: bool,
}

impl Default for Booster {
    fn default() -> Self {
        Self { active: false }
    }
}

#[derive(Component)]
pub struct Helm;

#[derive(Component)]
pub struct Pennant;

#[derive(Component)]
pub struct Sail;
