use bevy::prelude::*;

#[derive(Component)]
pub struct Cannon {
    pub power: f32,
    pub rig: Entity, // Entity of parent (ship)
}
#[derive(Component, Default)]
pub struct Aim {
    pub is_targeting: bool,
}

#[derive(Component, Default)]
pub struct CannonBall;
