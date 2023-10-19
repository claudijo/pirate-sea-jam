use bevy::prelude::*;

#[derive(Component, Default)]
pub struct GamepadTracker {
    pub touch_id: Option<u64>,
    // TODO: Add GamepadInteractionArea
}

#[derive(Component, Default)]
pub struct GamepadInteractionArea {
    pub rect: Rect,
}

#[derive(Component)]
pub struct DebugText;

#[derive(Component)]
pub struct TouchController {
    pub start_position: Vec2,
    // For convenient access to current touch position
    pub touch_position: Vec2,
}

#[derive(Component)]
pub struct TouchMarker {
    pub touch_id: u64,
}

#[derive(Component)]
pub struct TouchTrailMarker;
