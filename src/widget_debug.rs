mod systems;

use bevy::prelude::*;
use crate::widget_debug::systems::debug_buoys;

pub struct WidgetDebugPlugin;

impl Plugin for WidgetDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_buoys);
    }
}