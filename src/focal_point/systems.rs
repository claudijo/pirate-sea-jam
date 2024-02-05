use crate::floating_body::components::Position;
use crate::focal_point::resources::FocalPoint;
use crate::player::components::Player;
use crate::utils::vec2_extensions::Vec2Ext;
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn update_focal_point(
    player_query: Query<(&Player, &Position)>,
    local_players: Res<LocalPlayers>,
    mut focal_point: ResMut<FocalPoint>,
) {
    for (player, position) in &player_query {
        // Ignore non-local players
        if !local_players.0.contains(&player.handle) {
            continue;
        }

        focal_point.0 = position.0.extend_with_y(0.);
    }
}
