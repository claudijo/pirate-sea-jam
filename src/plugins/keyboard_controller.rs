use crate::game_state::GameState;
use crate::systems::player_input;
use bevy::prelude::*;

pub struct KeyboadControllerPlugin;

impl Plugin for KeyboadControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_input::turn_ship_using_keyboard,
                player_input::boost_ship_using_keyboard,
                player_input::reset_ship,
                player_input::reset_shooting_target,
                player_input::start_aiming_cannons_at_nearest_target_using_keyboard,
                player_input::tilt_aiming_cannons_using_keyboard,
                player_input::fire_aiming_cannons_using_keyboard,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
