use bevy::prelude::*;
use crate::systems::text;

pub struct TextOverlayPlugin;

impl Plugin for TextOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, text::display_text_overlay);
    }
}
