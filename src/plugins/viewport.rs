use bevy::prelude::*;
use crate::systems::viewport;

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, viewport::on_window_resize);
    }
}