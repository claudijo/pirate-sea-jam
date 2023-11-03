use bevy::prelude::*;

#[derive(Event)]
pub struct CameraControllerEvent {
    pub movement_delta: Vec2,
}