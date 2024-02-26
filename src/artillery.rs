use crate::artillery::components::{ArtilleryAiming, ArtilleryReady};
use crate::artillery::systems::{
    fire_artillery, register_start_aim_artillery_animations,
    register_stop_aim_artillery_animations, reload_artillery,
};
use crate::floating_body::systems::float;
use crate::physics::systems::integrate;
use crate::player::systems::update_player_position;
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod components;
mod resources;
mod systems;

const PORT_BACK_CANNON_TAG: &str = "Port back cannon";
const PORT_FRONT_CANNON_TAG: &str = "Port front cannon";
const STARBOARD_BACK_CANNON_TAG: &str = "Starboard back cannon";
const STARBOARD_FRONT_CANNON_TAG: &str = "Starboard front cannon";

pub struct ArtilleryPlugin;

impl Plugin for ArtilleryPlugin {
    fn build(&self, app: &mut App) {
        // Component candidates for roll back
        app.rollback_component_with_copy::<ArtilleryReady>();
        app.rollback_component_with_copy::<ArtilleryAiming>();

        app.add_systems(
            Startup,
            (
                register_start_aim_artillery_animations,
                register_stop_aim_artillery_animations,
            ),
        );

        app.add_systems(
            GgrsSchedule,
            (
                reload_artillery,
                fire_artillery
                    .after(update_player_position)
                    .after(reload_artillery)
                    .after(integrate)
                    .after(float),
            ),
        );
    }
}
