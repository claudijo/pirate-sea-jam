use bevy::prelude::*;
use crate::game_state::GameState;
use crate::systems::fluid_dynamics;
pub struct PontoonPlugin;

impl Plugin for PontoonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (fluid_dynamics::buoyancy).run_if(in_state(GameState::InGame)),
        );
    }
}
