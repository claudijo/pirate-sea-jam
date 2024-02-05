pub mod resources;
mod systems;

use crate::focal_point::resources::FocalPoint;
use crate::focal_point::systems::update_focal_point;
use crate::game_state::states::GameState;
use bevy::prelude::*;

pub struct FocalPointPlugin;

impl Plugin for FocalPointPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FocalPoint(Vec3::ZERO));

        app.add_systems(
            Update,
            update_focal_point.run_if(in_state(GameState::InGame)),
        );
    }
}
