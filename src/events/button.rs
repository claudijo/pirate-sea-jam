use bevy::prelude::*;

#[derive(Deref, Event)]
pub struct ButtonReleased(pub Entity);
