use crate::camera::systems::spawn_camera;
use crate::debug_fps::systems::{spawn_debug_fps, update_debug_fps};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod resources;
mod systems;

pub struct DebugFpsPlugin;

impl Plugin for DebugFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);

        app.add_systems(Startup, spawn_debug_fps.after(spawn_camera));

        app.add_systems(Update, update_debug_fps);
    }
}
