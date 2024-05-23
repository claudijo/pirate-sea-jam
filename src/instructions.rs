use crate::instructions::systems::display_control_keys;
use bevy::prelude::*;
use crate::camera::systems::spawn_camera;

mod systems;

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, display_control_keys.after(spawn_camera));
    }
}
