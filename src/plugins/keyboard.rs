use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use crate::systems::keyboard_input;
use bevy::prelude::*;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                keyboard_input::handle_turning_keys_pressed,
                keyboard_input::handle_turbo_booster_key_pressed,
                keyboard_input::handle_restart_game_key_pressed,
                keyboard_input::handle_fire_key_pressed,
                keyboard_input::handle_fire_key_released,
            )
                .run_if(resource_exists_and_equals(InputDevice::Mouse))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
