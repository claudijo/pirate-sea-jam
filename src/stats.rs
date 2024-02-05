use crate::args::run_conditions::p2p_mode;
use crate::game_state::states::GameState;
use crate::stats::resources::NetworkStatsTimer;
use crate::stats::systems::{print_events, print_network_stats};
use bevy::prelude::*;

mod resources;
mod systems;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkStatsTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, print_events.run_if(in_state(GameState::InGame)));

        app.add_systems(
            Update,
            print_network_stats.run_if(in_state(GameState::InGame).and_then(p2p_mode)),
        );
    }
}
