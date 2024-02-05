use crate::artillery::components::CannonsAreAiming;
use bevy::prelude::*;
use bevy_ggrs::GgrsApp;

pub mod components;
mod systems;

pub struct ArtilleryPlugin;

impl Plugin for ArtilleryPlugin {
    fn build(&self, app: &mut App) {
        // Component candidates for roll back
        app.rollback_component_with_copy::<CannonsAreAiming>();
    }
}
