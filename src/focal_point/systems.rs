use crate::focal_point::resources::FocalPoint;
use crate::player::components::Player;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn update_focal_point(
    player_query: Query<(&Player, &Transform)>,
    local_players: Res<LocalPlayers>,
    mut focal_point: ResMut<FocalPoint>,
) {
    for (player, transform) in &player_query {
        // Ignore non-local players
        if !local_players.0.contains(&player.handle) {
            continue;
        }

        focal_point.0 = transform.translation;
        focal_point.0.y = 0.;
    }
}
