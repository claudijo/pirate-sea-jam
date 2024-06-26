mod systems;

use crate::widget_debug::systems::{debug_buoys, debug_physics_particle};
use bevy::prelude::*;

pub struct WidgetDebugPlugin;

impl Plugin for WidgetDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (debug_buoys, debug_physics_particle));
    }
}
