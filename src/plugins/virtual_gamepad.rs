use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use crate::systems::virtual_gamepad_input;
use bevy::prelude::*;

pub struct VirtualGamepadPlugin;

impl Plugin for VirtualGamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, virtual_gamepad_input::show_debug_text);

        app.add_systems(
            OnEnter(GameState::InGame),
            (
                virtual_gamepad_input::init_movement_gamepad,
                virtual_gamepad_input::spawn_cross_button,
                virtual_gamepad_input::spawn_circle_button,
            )
                .run_if(resource_exists_and_equals(InputDevice::Touch)),
        );

        app.add_systems(
            Update,
            (
                virtual_gamepad_input::track_virtual_joystick,
                virtual_gamepad_input::capture_virtual_joystick,
                virtual_gamepad_input::release_virtual_joystick,
                virtual_gamepad_input::arrange_knob_trail_dots,
                virtual_gamepad_input::handle_cross_button_interactions,
                virtual_gamepad_input::handle_cross_button_release,
                virtual_gamepad_input::handle_circle_button_interaction,
            )
                .run_if(resource_exists_and_equals(InputDevice::Touch))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
