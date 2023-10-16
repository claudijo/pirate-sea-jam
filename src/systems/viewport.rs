use bevy::prelude::*;
use bevy::window::WindowResized;

pub fn on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.iter() {
        // When resolution is being changed
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}