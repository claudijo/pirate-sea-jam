use bevy::input::mouse::MouseMotion;
use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use bevy::prelude::*;
use crate::plugins::orbiting_camera::OrbitMotion;


fn orbit_camera(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut orbit_motion_event_writer: EventWriter<OrbitMotion>,
) {
    for mouse_motion_event in &mut mouse_motion_event_reader {
        orbit_motion_event_writer.send(OrbitMotion { delta: mouse_motion_event.delta });
    }
}


pub struct MouseInputPlugin;

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (orbit_camera,)
                .run_if(resource_exists_and_equals(InputDevice::Mouse))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
