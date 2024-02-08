use crate::artillery::components::ArtilleryReady;
use crate::artillery::systems::{fire_artillery, reload_artillery};
use crate::player::systems::update_player_position;
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod components;
mod systems;

pub struct ArtilleryPlugin;

impl Plugin for ArtilleryPlugin {
    fn build(&self, app: &mut App) {
        // Component candidates for roll back
        app.rollback_component_with_copy::<ArtilleryReady>();

        app.add_systems(
            GgrsSchedule,
            (
                reload_artillery,
                fire_artillery
                    .after(update_player_position)
                    .after(reload_artillery),
            ),
        );
    }
}
