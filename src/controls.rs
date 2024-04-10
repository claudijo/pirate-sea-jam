use crate::controls::components::{
    checksum_helm_rotation_speed, HelmRotationalSpeed,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod components;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        // Component candidates for roll back
        app.rollback_component_with_copy::<HelmRotationalSpeed>();

        app.checksum_component::<HelmRotationalSpeed>(checksum_helm_rotation_speed);
    }
}
