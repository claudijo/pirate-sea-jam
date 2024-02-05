use crate::light::systems::spawn_light;
use bevy::prelude::*;

mod systems;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}
