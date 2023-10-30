use bevy::prelude::*;

#[derive(Component)]
pub struct Cannon {
    pub power: f32,
    pub max_tilt: f32,
    pub default_tilt: f32,
    pub tilt_factor: f32,

    // Entity of parent for querying properties such as velocity
    pub rig: Option<Entity>,
}

impl Default for Cannon {
    fn default() -> Self {
        Self {
            power: 1.,
            max_tilt: 30.0,
            rig: None,
            default_tilt: 0.,
            tilt_factor: 0.,
        }
    }
}

#[derive(Component, Default)]
pub struct CannonGunPowder {
    pub is_lit: bool,
}

#[derive(Component, Default)]
pub struct Aim {
    pub is_targeting: bool,
}

#[derive(Component, Default)]
pub struct Tilt {
    pub angle: f32,
}

#[derive(Component, Default)]
pub struct CannonBall;
