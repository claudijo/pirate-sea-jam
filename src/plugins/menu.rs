use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use crate::systems::menu;
use bevy::prelude::*;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), menu::spawn_main_menu)
            .add_systems(OnExit(GameState::SplashScreen), menu::despawn_main_menu)
            .add_systems(
                Update,
                menu::handle_main_menu_interactions.run_if(in_state(GameState::SplashScreen)),
            );

        app.add_systems(
            OnEnter(GameState::InGame),
            menu::spawn_restart_game_button.run_if(resource_exists_and_equals(InputDevice::Touch)),
        );

        app.add_systems(
            Update,
            menu::handle_restart_game_button_interactions
                .run_if(resource_exists_and_equals(InputDevice::Touch))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
