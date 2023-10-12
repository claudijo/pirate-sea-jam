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
            stability: 6.,
        }
    }
}

#[derive(Component, Default)]
pub struct ShipTurnRate {
    pub value: f32,
}

#[derive(Component, Default)]
pub struct ShipBooster {
    pub active: bool,
}

#[derive(Component)]
pub struct ShipHelm;

#[derive(Component)]
pub struct ShipFlag {
    pub rig: Option<Entity>,
}

#[derive(Component)]
pub struct ShipSail;
