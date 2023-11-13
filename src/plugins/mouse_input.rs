use crate::components::ship::PlayerShip;
use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::game_state::GameState;
use crate::plugins::orbiting_camera::OrbitMotion;
use crate::resources::player::InputDevice;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

fn orbit_camera(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut orbit_motion_event_writer: EventWriter<OrbitMotion>,
) {
    for mouse_motion_event in mouse_motion_event_reader.read() {
        orbit_motion_event_writer.send(OrbitMotion {
            delta: mouse_motion_event.delta,
        });
    }
}

fn handle_fire_button(
    buttons: Res<Input<MouseButton>>,
    ship_query: Query<Entity, With<PlayerShip>>,
    mut aim_cannon_event_writer: EventWriter<AimCannonEvent>,
    mut fire_cannon_event_writer: EventWriter<FireCannonEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for entity in &ship_query {
            aim_cannon_event_writer.send(AimCannonEvent(entity));
        }
    }
    if buttons.just_released(MouseButton::Left) {
        for entity in &ship_query {
            fire_cannon_event_writer.send(FireCannonEvent(entity));
        }
    }
}

pub struct MouseInputPlugin;

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (orbit_camera, handle_fire_button)
                .run_if(resource_exists_and_equals(InputDevice::Mouse))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
