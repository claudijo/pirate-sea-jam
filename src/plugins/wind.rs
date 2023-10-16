use crate::game_state::GameState;
use crate::systems::wind::spawn_wind;
use bevy::prelude::*;

pub struct WindPlugin;

impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_wind);
    }
}
