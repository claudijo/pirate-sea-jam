use crate::game_state::GameState;
use bevy::prelude::*;
use crate::systems::light;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), light::spawn_light);
    }
}


