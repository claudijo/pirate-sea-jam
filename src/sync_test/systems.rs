use crate::args::resources::Args;
use crate::connection::systems::RollbackConfig;
use crate::connection::{FPS, INPUT_DELAY, MAX_PREDICTION};
use crate::game_state::states::GameState;
use bevy::prelude::*;
use bevy_ggrs::ggrs;

pub fn start_sync_test_session(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    args: Res<Args>,
) {
    info!("Starting synctest session");
    let mut session_builder = ggrs::SessionBuilder::<RollbackConfig>::new()
        .with_num_players(args.num_players)
        .with_max_prediction_window(MAX_PREDICTION)
        .expect("prediction window can't be 0")
        .with_fps(FPS)
        .expect("FPS can't be 0")
        .with_input_delay(INPUT_DELAY)
        // GGRS will simulate a rollback every frame and re-simulate the last n states, where n is the given
        // check_distance. All expensive operations are skipped if the check distance is 0, enabling use of synctest
        // mode for general local play.
        .with_check_distance(args.check_distance);

    for i in 0..args.num_players {
        session_builder = session_builder
            .add_player(ggrs::PlayerType::Local, i)
            .expect("failed to add player");
    }

    let ggrs_session = session_builder
        .start_synctest_session()
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::SyncTest(ggrs_session));
    next_state.set(GameState::InGame);
}
