use bevy::prelude::*;

#[derive(Event)]
pub struct OrbitMotion {
    pub delta: Vec2,
}
