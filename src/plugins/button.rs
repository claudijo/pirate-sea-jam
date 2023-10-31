// Concept from https://github.com/Leinnan/bevy_button_released_plugin/blob/master/src/lib.rs

use crate::events::button::ButtonReleasedEvent;
use crate::systems::button::react_to_touch_button_release;
use bevy::prelude::*;

pub struct ButtonsReleasedPlugin;

impl Plugin for ButtonsReleasedPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonReleasedEvent>()
            .add_systems(Update, react_to_touch_button_release);
    }
}
