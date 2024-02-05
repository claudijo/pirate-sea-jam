pub mod components;
pub mod systems;

use crate::game_state::states::GameState;
use crate::player::components::Player;
use crate::player::systems::{
    animate_flag, animate_helm, apply_inputs, spawn_players, update_position, update_velocity,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub const LINEAR_ACCELERATION: f32 = 10.;
pub const ANGULAR_ACCELERATION: f32 = 4.;
pub const MAX_LINEAR_SPEED: f32 = 6.;
pub const MAX_ANGULAR_SPEED: f32 = 1.;
pub const LINEAR_DAMPING: f32 = 1.;
pub const ANGULAR_DAMPING: f32 = 0.1;
pub const TRACTION: f32 = 1.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_players);
        app.add_systems(
            GgrsSchedule,
            (apply_inputs, update_velocity, update_position).chain(),
        );

        app.add_systems(
            Update,
            (animate_helm, animate_flag).run_if(in_state(GameState::InGame)),
        );

        // Registered all components that needs to be restored when rollback entities are restored
        app.rollback_component_with_copy::<Player>();
        app.rollback_component_with_clone::<Sprite>();
        app.rollback_component_with_clone::<GlobalTransform>();
        app.rollback_component_with_clone::<Handle<Image>>();
        app.rollback_component_with_clone::<Visibility>();
        app.rollback_component_with_clone::<InheritedVisibility>();
        app.rollback_component_with_clone::<Transform>();
        app.rollback_component_with_clone::<ViewVisibility>();
        app.rollback_component_with_clone::<Name>();
        app.rollback_component_with_clone::<Handle<Scene>>();
    }
}
