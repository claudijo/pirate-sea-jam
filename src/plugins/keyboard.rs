use crate::game_state::GameState;
use crate::systems::keyboard_input;
use bevy::prelude::*;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                keyboard_input::turn_ship,
                keyboard_input::boost_ship,
                keyboard_input::handle_restart_game_key_pressed,
                keyboard_input::handle_fire_key_pressed,
                keyboard_input::handle_fire_key_released,
                // keyboard_input::start_aiming_cannons_at_nearest_target,
                // keyboard_input::tilt_aiming_cannons,
                // keyboard_input::fire_aiming_cannons,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}