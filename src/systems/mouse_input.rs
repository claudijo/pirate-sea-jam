use crate::plugins::orbiting_camera::OrbitEvent;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub fn orbit_camera(
    mut motion_event_reader: EventReader<MouseMotion>,
    mut orbit_event_writer: EventWriter<OrbitEvent>,
) {
    for event in &mut motion_event_reader {
        orbit_event_writer.send(OrbitEvent { delta: event.delta });
    }
}
