use crate::inputs::systems::{read_local_inputs, read_mouse_input};
use bevy::prelude::*;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::ReadInputs;

mod systems;

// pub const INPUT_UP: u8 = 1 << 0;
// pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;

pub fn turn_action_from_input(input_and_status: (u8, InputStatus)) -> i32 {
    let input = match input_and_status.1 {
        InputStatus::Confirmed => input_and_status.0,
        InputStatus::Predicted => input_and_status.0,
        InputStatus::Disconnected => 0, // disconnected players do nothing
    };

    let mut turn: i32 = 0;

    if input & INPUT_RIGHT != 0 {
        turn += 1;
    }
    if input & INPUT_LEFT != 0 {
        turn -= 1;
    }

    turn
}

pub fn fire(input: u8) -> bool {
    input & INPUT_FIRE != 0
}

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(ReadInputs, read_local_inputs);

        app.add_systems(Update, read_mouse_input);
    }
}
