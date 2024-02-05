use crate::connection::systems::RollbackConfig;
use crate::stats::resources::NetworkStatsTimer;
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

pub fn print_network_stats(
    time: Res<Time>,
    mut timer: ResMut<NetworkStatsTimer>,
    p2p_session: Res<Session<RollbackConfig>>,
) {
    // print only when timer runs out
    if timer.0.tick(time.delta()).just_finished() {
        match p2p_session.as_ref() {
            Session::P2P(s) => {
                let num_players = s.num_players();
                for i in 0..num_players {
                    if let Ok(stats) = s.network_stats(i) {
                        println!("NetworkStats for player {}: {:?}", i, stats);
                    }
                }
            }
            _ => panic!("This examples focuses on p2p."),
        }
    }
}

pub(crate) fn print_events(mut session: ResMut<Session<RollbackConfig>>) {
    if let Session::P2P(s) = session.as_mut() {
        for event in s.events() {
            match event {
                GgrsEvent::Disconnected { .. } | GgrsEvent::NetworkInterrupted { .. } => {
                    warn!("GGRS event: {event:?}")
                }
                GgrsEvent::DesyncDetected {
                    local_checksum,
                    remote_checksum,
                    frame,
                    ..
                } => {
                    error!("Desync on frame {frame}. Local checksum: {local_checksum:X}, remote checksum: {remote_checksum:X}");
                }
                _ => info!("GGRS event: {event:?}"),
            }
        }
    }
}
