use crate::controls::components::{
    checksum_yaw_rotation_speed, YawRotationalSpeed,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod components;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        // Component candidates for roll back
        app.rollback_component_with_copy::<YawRotationalSpeed>();

        app.checksum_component::<YawRotationalSpeed>(checksum_yaw_rotation_speed);
    }
}
