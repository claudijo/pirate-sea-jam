use bevy::prelude::*;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct ResetGameButton;

#[derive(Component)]
pub struct CrossGamepadButton;

#[derive(Component)]
pub struct CircleGamepadButton;

#[derive(Component, Default)]
pub struct ReleasableButton {
    pub last_state: Interaction,
}