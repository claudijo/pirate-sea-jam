mod systems;

use crate::args::run_conditions::sync_test_mode;
use crate::game_state::states::GameState;
use crate::sync_test::systems::start_sync_test_session;
use bevy::prelude::*;

pub struct SyncTestPlugin;

impl Plugin for SyncTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            start_sync_test_session
                .run_if(in_state(GameState::Matchmaking).and_then(sync_test_mode)),
        );
    }
}
