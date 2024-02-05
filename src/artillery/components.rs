use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub struct CannonsAreAiming(pub bool);

#[derive(Component, Clone, Copy, Default)]
pub struct CannonBall;
