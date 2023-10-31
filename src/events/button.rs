use bevy::prelude::*;

#[derive(Deref, Event)]
pub struct ButtonReleasedEvent(pub Entity);
