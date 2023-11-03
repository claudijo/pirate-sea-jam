use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::events::camera::CameraControllerEvent;

pub fn orbit_camera(
    mut motion_event_reader: EventReader<MouseMotion>,
    mut camera_controller_event_writer: EventWriter<CameraControllerEvent>,
) {
    for event in &mut motion_event_reader {
        camera_controller_event_writer.send(CameraControllerEvent {
            movement_delta: event.delta,
        });
    }
}