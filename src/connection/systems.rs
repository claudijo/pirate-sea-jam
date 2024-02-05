use crate::args::resources::Args;
use crate::connection::{FPS, INPUT_DELAY, MAX_PREDICTION};
use crate::game_state::states::GameState;
use bevy::prelude::*;
use bevy_ggrs::{ggrs, GgrsConfig};
use bevy_matchbox::prelude::*;

// The first generic parameter, u8, is the input type: 4-directions + fire fits
// easily in a single byte
// The second parameter is the address type of peers: Matchbox' WebRtcSocket
// addresses are called `PeerId`s
pub type RollbackConfig = GgrsConfig<u8, PeerId>;

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://192.168.100.158:3536/rogue_waves?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_state: ResMut<NextState<GameState>>,
    args: Res<Args>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    if players.len() < args.num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<RollbackConfig>::new()
        .with_num_players(args.num_players)
        // (optional) set how often to exchange state checksums for their confirmed frames (frames where we have
        // complete information about inputs for all players).
        .with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 10 })
        // (optional) set max prediction window
        .with_max_prediction_window(MAX_PREDICTION)
        .expect("prediction window can't be 0")
        .with_fps(FPS)
        .expect("FPS can't be 0")
        // (optional) set input delay for the local player
        .with_input_delay(INPUT_DELAY);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
    next_state.set(GameState::InGame);
}
