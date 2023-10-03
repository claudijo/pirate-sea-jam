use crate::game_state::GameState;
use crate::systems::light;
use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), light::spawn_light);
    }
}
