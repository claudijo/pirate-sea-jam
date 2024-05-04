mod components;
mod systems;

use crate::game_state::states::GameState;
use crate::menu::systems::{despawn_main_menu, handle_main_menu_interactions, spawn_main_menu};
use bevy::prelude::*;

pub struct MenuPlugin;

pub const START_BUTTON_NORMAL: Color = Color::rgb(0.9, 0.45, 0.21);
pub const START_BUTTON_HOVER: Color = Color::rgb(0.87, 0.36, 0.18);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_main_menu)
            .add_systems(OnExit(GameState::SplashScreen), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_interactions.run_if(in_state(GameState::SplashScreen)),
            );
    }
}
