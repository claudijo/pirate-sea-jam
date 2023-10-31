use bevy::prelude::*;

#[derive(Deref, Event)]
pub struct AimCannonEvent(pub Entity);

#[derive(Deref, Event)]
pub struct FireCannonEvent(pub Entity);
