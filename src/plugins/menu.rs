use crate::game_state::GameState;
use crate::systems::menu;
use bevy::prelude::*;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), menu::setup_start_menu)
            .add_systems(OnExit(GameState::SplashScreen), menu::tear_down_start_menu)
            .add_systems(
                Update,
                menu::update_start_menu.run_if(in_state(GameState::SplashScreen)),
            );
    }
}
