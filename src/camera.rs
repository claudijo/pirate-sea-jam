use crate::camera::systems::spawn_camera;
use bevy::prelude::*;

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
