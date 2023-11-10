use crate::components::ship::{PlayerShip, ShipBooster, ShipRudder};
use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::game_state::GameState;
use crate::libs::plugins::virtual_joystick::{VirtualJoystickMotion, VirtualJoystickPosition};
use crate::plugins::orbiting_camera::{OrbitMotion, OrbitingCamera};
use crate::plugins::virtual_gamepad::{
    ButtonId, GamepadButtonPressed, GamepadButtonReleased, CAMERA_JOYSTICK,
    PLAYER_SHIP_STEERING_JOYSTICK,
};
use crate::resources::player::InputDevice;
use crate::systems::movement::{RATE_OF_ROTATION, TURN_RATE_LIMIT};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

fn handle_camera_joystick_movement(
    mut virtual_joystick_motion_event_reader: EventReader<VirtualJoystickMotion>,
    mut orbit_motion_event_writer: EventWriter<OrbitMotion>,
) {
    for virtual_joystick_motion in &mut virtual_joystick_motion_event_reader {
        if virtual_joystick_motion.id == CAMERA_JOYSTICK {
            orbit_motion_event_writer.send(OrbitMotion {
                delta: virtual_joystick_motion.delta,
            });
        }
    }
}

fn handle_player_ship_joystick_position(
    mut rudder_query: Query<&mut ShipRudder>,
    ship_query: Query<&Transform, With<PlayerShip>>,
    virtual_joystick_position: ResMut<VirtualJoystickPosition>,
    orbiting_camera_query: Query<&OrbitingCamera>,
    time: Res<Time>,
) {
    for mut rudder in &mut rudder_query {
        if let Some(controller_direction) = virtual_joystick_position
            .by_joystick_id
            .get(&PLAYER_SHIP_STEERING_JOYSTICK)
        {
            rudder.is_turning = true;

            for camera in &orbiting_camera_query {
                let controller_direction =
                    Vec2::new(camera.yaw.cos(), camera.yaw.sin()).rotate(*controller_direction);

                for ship_transform in &ship_query {
                    let ship_forward = ship_transform.forward();
                    let controller_angle = controller_direction.angle_between(ship_forward.xz());
                    if controller_angle < 0. {
                        let new_angle = rudder.turn_rate - time.delta_seconds() * RATE_OF_ROTATION;
                        rudder.turn_rate = new_angle.max(-TURN_RATE_LIMIT);
                    } else {
                        let new_angle = rudder.turn_rate + time.delta_seconds() * RATE_OF_ROTATION;
                        rudder.turn_rate = new_angle.min(TURN_RATE_LIMIT);
                    }
                }
            }
        } else {
            rudder.is_turning = false;
        }
    }
}

fn handle_gamepad_button_pressed(
    mut ship_query: Query<(Entity, &mut ShipBooster), With<PlayerShip>>,
    mut gamepad_button_pressed_event_reader: EventReader<GamepadButtonPressed>,
    mut aim_cannon_event_writer: EventWriter<AimCannonEvent>,
) {
    for gamepad_pressed in gamepad_button_pressed_event_reader.iter() {
        match gamepad_pressed.id {
            ButtonId::South => {
                for (entity, _) in &ship_query {
                    aim_cannon_event_writer.send(AimCannonEvent(entity));
                }
            }

            ButtonId::East => {
                for (_, mut booster) in &mut ship_query {
                    booster.active = true;
                }
            }
        }
    }
}

fn handle_gamepad_button_released(
    ship_query: Query<Entity, With<PlayerShip>>,
    mut gamepad_button_released_event_reader: EventReader<GamepadButtonReleased>,
    mut fire_cannon_event_writer: EventWriter<FireCannonEvent>,
) {
    for gamepad_released in gamepad_button_released_event_reader.iter() {
        if gamepad_released.id == ButtonId::South {
            for entity in &ship_query {
                fire_cannon_event_writer.send(FireCannonEvent(entity));
            }
        }
    }
}

pub struct VirtualGamepadInputPlugin;

impl Plugin for VirtualGamepadInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_camera_joystick_movement,
                handle_player_ship_joystick_position,
                handle_gamepad_button_pressed,
                handle_gamepad_button_released,
            )
                .run_if(resource_exists_and_equals(InputDevice::Touch))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
