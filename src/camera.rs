use crate::camera::systems::{
    grab_pointer, release_pointer, release_pointer_on_escape, spawn_camera,
};
use crate::game_state::states::GameState;
use bevy::prelude::*;

pub mod systems;
pub mod resources;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);

        app.add_systems(OnEnter(GameState::InGame), grab_pointer);
        app.add_systems(OnExit(GameState::InGame), release_pointer);

        app.add_systems(
            Update,
            release_pointer_on_escape.run_if(in_state(GameState::InGame)),
        );
    }
}
