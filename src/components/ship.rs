use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerShip;

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
            maneuverability: 1.4,
            booster_power: 160.,
            stability: 6.,
        }
    }
}

#[derive(Component, Default)]
pub struct ShipRudder {
    pub turn_rate: f32,
    pub is_turning: bool,
}

#[derive(Component, Default)]
pub struct ShipBooster {
    pub active: bool,
}

#[derive(Component)]
pub struct ShipHelm;

#[derive(Component)]
pub struct ShipSail;

#[derive(Component)]
pub struct ShipFlag {
    pub rig: Entity,
}
