use bevy::prelude::*;

#[derive(Component)]
pub struct Lifespan {
    pub ttl: Timer,
}
