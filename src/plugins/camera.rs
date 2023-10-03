use crate::game_state::GameState;
use crate::systems::camera;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), camera::spawn_camera);
    }
}
