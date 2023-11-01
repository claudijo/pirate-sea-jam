use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use crate::systems::text;
use bevy::prelude::*;

pub struct TextOverlayPlugin;

impl Plugin for TextOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            text::display_control_keys.run_if(resource_exists_and_equals(InputDevice::Mouse)),
        );
    }
}
