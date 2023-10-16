use crate::game_state::GameState;
use crate::resources::wave_machine::WaveMachine;
use crate::systems::{fluid_dynamics, ocean};
use bevy::prelude::*;

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveMachine {
            time_scale: 0.4,
            sample_count: 4,
        })
        .add_systems(OnEnter(GameState::SplashScreen), ocean::spawn_ocean)
        .add_systems(
            Update,
            (
                fluid_dynamics::make_waves.run_if(in_state(GameState::SplashScreen)),
                fluid_dynamics::make_waves.run_if(in_state(GameState::InGame)),
            ),
        );
    }
}
