use bevy::prelude::*;

#[derive(Deref, Event)]
pub struct RestartGameEvent(pub Entity);
