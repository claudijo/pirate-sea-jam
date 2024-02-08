use crate::floating_body::components::{
    checksum_linear_velocity, checksum_position, checksum_yaw, checksum_yaw_rotation_speed,
    LinearVelocity, Position, Yaw, YawRotationalSpeed,
};
use crate::floating_body::systems::float;
use crate::player::systems::update_player_position;
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub mod components;
pub mod systems;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(GgrsSchedule, float.before(update_player_position));

        // Component candidates for roll back
        app.rollback_component_with_copy::<LinearVelocity>();
        app.rollback_component_with_copy::<YawRotationalSpeed>();
        app.rollback_component_with_copy::<Position>();
        app.rollback_component_with_copy::<Yaw>();

        app.checksum_component::<LinearVelocity>(checksum_linear_velocity);
        app.checksum_component::<YawRotationalSpeed>(checksum_yaw_rotation_speed);
        app.checksum_component::<Position>(checksum_position);
        app.checksum_component::<Yaw>(checksum_yaw);
    }
}
