use crate::controls::components::{checksum_wheel_turn_ratio, WheelTurnRatio};
use bevy::prelude::*;
use bevy_ggrs::GgrsApp;

pub mod components;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        // Component candidates for roll back
        app.rollback_component_with_copy::<WheelTurnRatio>();

        app.checksum_component::<WheelTurnRatio>(checksum_wheel_turn_ratio);
    }
}
