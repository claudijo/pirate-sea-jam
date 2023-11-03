use crate::systems::camera;
use bevy::prelude::*;
use crate::game_state::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::spawn_camera);

        app.add_systems(OnEnter(GameState::InGame), camera::grab_pointer);
        app.add_systems(OnExit(GameState::InGame), camera::release_pointer);


        app.add_systems(
            Update,
            (
                camera::follow_ship,
                camera::orbit,
                camera::release_pointer_on_escape,
            ).run_if(in_state(GameState::InGame))
        );
    }
}
