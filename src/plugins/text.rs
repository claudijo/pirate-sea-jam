use crate::systems::text;
use bevy::prelude::*;

pub struct TextOverlayPlugin;

impl Plugin for TextOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, text::display_control_keys);
    }
}
