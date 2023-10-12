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
                artillery::fire_cannons,
                artillery::despawn_cannon_ball,
                artillery::rewind_cannon_tilt,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
