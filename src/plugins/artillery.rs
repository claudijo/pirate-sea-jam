use crate::game_state::GameState;
use crate::systems::artillery;
use bevy::app::Plugin;
use bevy::prelude::*;

pub struct ArsenalPlugin;

impl Plugin for ArsenalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                artillery::despawn_cannon_ball,
                artillery::handle_cannon_aim_event,
                artillery::handle_cannon_fire_event,
                // artillery::tilt_cannon,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
