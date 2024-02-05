use crate::args::run_conditions::p2p_mode;
use crate::connection::systems::{start_matchbox_socket, wait_for_players};
use crate::game_state::states::GameState;
use bevy::prelude::*;

pub mod systems;

pub const MAX_PREDICTION: usize = 12;
pub const FPS: usize = 60;
pub const INPUT_DELAY: usize = 2;

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Matchmaking),
            start_matchbox_socket.run_if(p2p_mode),
        );

        app.add_systems(
            Update,
            wait_for_players.run_if(in_state(GameState::Matchmaking).and_then(p2p_mode)),
        );
    }
}
