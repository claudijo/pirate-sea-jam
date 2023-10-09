use crate::game_state::GameState;
use crate::systems::arsenal;
use bevy::app::Plugin;
use bevy::prelude::*;

pub struct ArsenalPlugin;

impl Plugin for ArsenalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                arsenal::fire_cannons,
                arsenal::despawn_cannon_ball,
            ).run_if(in_state(GameState::InGame)),
        );
    }
}
