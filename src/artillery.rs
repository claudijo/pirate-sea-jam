use crate::artillery::components::{ArtilleryAiming, ArtilleryReady};
use crate::artillery::systems::{
    despawn_projectile, register_start_aim_artillery_animations,
    register_stop_aim_artillery_animations, start_aim_artillery, stop_aim_and_fire_artillery,
};
use crate::physics::systems::update_orientation;
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod components;
mod resources;
mod systems;

pub const PORT_BACK_CANNON_TAG: &str = "Port back cannon";
pub const PORT_FRONT_CANNON_TAG: &str = "Port front cannon";
pub const STARBOARD_BACK_CANNON_TAG: &str = "Starboard back cannon";
pub const STARBOARD_FRONT_CANNON_TAG: &str = "Starboard front cannon";

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
                start_aim_artillery.after(update_orientation),
                stop_aim_and_fire_artillery.after(start_aim_artillery),
                despawn_projectile,
            ),
        );
    }
}
