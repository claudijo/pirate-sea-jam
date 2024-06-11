pub mod components;
pub mod systems;

use crate::game_state::states::GameState;
use crate::physics::systems::update_aerodynamic_force;
use crate::player::components::Player;
use crate::player::systems::{
    animate_flag, animate_sail_trim, animate_wheel_turn, apply_inputs, spawn_players,
    update_focal_point, update_hull_drag, update_rudder, update_sail_trim_ratio,
    update_wheel_turn_ratio,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

pub const WHEEL_TURN_ACCELERATION: f32 = 4.;
pub const WHEEL_TURN_DAMPING: f32 = 0.1;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_players);
        app.add_systems(
            GgrsSchedule,
            (
                apply_inputs,
                update_rudder,
                update_hull_drag,
                update_wheel_turn_ratio,
                update_sail_trim_ratio,
            )
                .chain()
                .before(update_aerodynamic_force),
        );

        app.add_systems(
            Update,
            (animate_sail_trim, animate_wheel_turn, animate_flag)
                .run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            Update,
            update_focal_point.run_if(in_state(GameState::InGame)),
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
