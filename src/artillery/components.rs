use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub struct ArtilleryReady(pub bool);

#[derive(Component, Clone, Copy, Default)]
pub struct ArtilleryAiming(pub bool);

#[derive(Component, Clone, Copy, Default)]
pub struct Projectile;

#[derive(Component, Clone, Copy, Default)]
pub struct Artillery {
    pub muzzle_velocity: f32,
}
