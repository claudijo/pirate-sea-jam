use bevy::prelude::*;

#[derive(Component)]
pub struct Cannon {
    pub power: f32,
    pub max_tilt: f32,
    pub default_tilt: f32,
    pub tilt_factor: f32,

    pub tilt_torque: f32,
    pub rig: Entity,  // Entity of parent (ship)
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
