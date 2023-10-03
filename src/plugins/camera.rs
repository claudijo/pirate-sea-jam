use crate::game_state::GameState;
use bevy::prelude::*;
use crate::systems::camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), camera::spawn_camera);
    }
}
