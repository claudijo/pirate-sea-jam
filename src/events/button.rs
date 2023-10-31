use bevy::prelude::*;

#[derive(Event)]
pub struct ButtonReleasedEvent {
    pub source: Entity,
}
