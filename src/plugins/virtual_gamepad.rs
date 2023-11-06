use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use crate::systems::virtual_gamepad_input;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::libs::plugins::virtual_joystick::{Joystick, VirtualJoystickMotion};

pub const PLAYER_SHIP_STEERING_JOYSTICK: u8 = 0;
pub const CAMERA_JOYSTICK: u8 = 1;

fn spawn_left_stick(mut commands: Commands) {
    commands.spawn((
        Joystick::with_id(PLAYER_SHIP_STEERING_JOYSTICK),
        RelativeCursorPosition::default(),
        NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_right_stick(mut commands: Commands) {
    commands.spawn((
        Joystick::with_id(CAMERA_JOYSTICK).hide(),
        RelativeCursorPosition::default(),
        NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                right: Val::Px(0.),
                ..default()
            },
            ..default()
        },
    ));
}

pub struct VirtualGamepadPlugin;

impl Plugin for VirtualGamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_left_stick, spawn_right_stick)
                .run_if(resource_exists_and_equals(InputDevice::Touch)),
        );
    }
}
