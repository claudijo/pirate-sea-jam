use bevy::prelude::*;
use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use crate::systems::mouse_input;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mouse_input::orbit_camera,
            )
                .run_if(resource_exists_and_equals(InputDevice::Mouse))
                .run_if(in_state(GameState::InGame)),
        );
    }
}