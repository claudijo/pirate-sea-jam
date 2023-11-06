use bevy::input::mouse::MouseMotion;
use crate::game_state::GameState;
use crate::resources::player::InputDevice;
use bevy::prelude::*;
use crate::libs::plugins::virtual_joystick::VirtualJoystickMotion;
use crate::plugins::orbiting_camera::OrbitMotion;
use crate::plugins::virtual_gamepad::CAMERA_JOYSTICK;

fn handle_camera_joystick_movement(
    mut virtual_joystick_motion_event_reader: EventReader<VirtualJoystickMotion>,
    mut orbit_motion_event_writer: EventWriter<OrbitMotion>,
) {
    for virtual_joystick_motion_event in &mut virtual_joystick_motion_event_reader {
        if virtual_joystick_motion_event.id == CAMERA_JOYSTICK {
            orbit_motion_event_writer.send(OrbitMotion { delta: virtual_joystick_motion_event.delta });
        }
    }
}

fn handle_player_ship_steering_joystick_movement(

) {

}

pub struct VirtualGamepadInputPlugin;

impl Plugin for VirtualGamepadInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_camera_joystick_movement, handle_player_ship_steering_joystick_movement,)
                .run_if(resource_exists_and_equals(InputDevice::Touch))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
