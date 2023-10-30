use bevy::prelude::*;

#[derive(Event)]
pub struct AimCannonEvent {
    pub source: Entity, // Ship entity
}

#[derive(Event)]
pub struct FireCannonEvent {
    pub source: Entity, // Ship entity
}
