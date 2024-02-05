use bevy::prelude::*;

use clap::Parser;

#[derive(Parser, Resource, Debug, Clone)]
pub struct Args {
    // runs the game in sync test mode / local player mode
    #[clap(long, default_value = "true")]
    pub sync_test: bool,

    // Set to 2 for doing sync tests, 0 for optimized single player game
    #[clap(long, default_value = "0")]
    pub check_distance: usize,

    #[clap(long, default_value = "1")]
    pub num_players: usize,
}
