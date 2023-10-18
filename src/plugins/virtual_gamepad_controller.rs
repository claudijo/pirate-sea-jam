use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use crate::systems::virtual_gamepad_input;
use bevy::prelude::*;

pub struct VirtualGamepadControllerPlugin;

impl Plugin for VirtualGamepadControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, virtual_gamepad_input::show_debug_text);

        app.add_systems(
            OnEnter(GameState::InGame),
            virtual_gamepad_input::init_movement_gamepad
                .run_if(resource_exists_and_equals(InputDevice::Touch)),
        );

        app.add_systems(
            Update,
            (
                virtual_gamepad_input::capture_virtual_gamepad,
                virtual_gamepad_input::track_virtual_gamepad,
                virtual_gamepad_input::release_virtual_gamepad,
                virtual_gamepad_input::arrange_knob_trail_dots,
            )
                .run_if(resource_exists_and_equals(InputDevice::Touch))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
