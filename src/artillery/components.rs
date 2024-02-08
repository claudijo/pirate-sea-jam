use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub struct ArtilleryReady(pub bool);

#[derive(Component, Clone, Copy, Default)]
pub struct Projectile;
