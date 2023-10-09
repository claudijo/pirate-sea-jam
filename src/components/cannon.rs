use bevy::prelude::*;


#[derive(Component, Default)]
pub struct Cannon {
    pub is_lit: bool,
    pub power: f32,

    // Entity of the rigid body parent for querying properties such as velocity
    pub rig: Option<Entity>,
}

#[derive(Component, Default)]
pub struct CannonBall;
