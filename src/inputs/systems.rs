use crate::connection::systems::RollbackConfig;
use crate::inputs::{INPUT_FIRE, INPUT_LEFT, INPUT_RIGHT};
use crate::orbiting_camera::events::OrbitMotion;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ggrs::{LocalInputs, LocalPlayers};

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0u8;
        if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::Space, KeyCode::Return]) {
            input |= INPUT_FIRE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<RollbackConfig>(local_inputs));
}

pub fn read_mouse_input(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut orbit_motion_event_writer: EventWriter<OrbitMotion>,
) {
    for mouse_motion_event in mouse_motion_event_reader.read() {
        orbit_motion_event_writer.send(OrbitMotion {
            delta: mouse_motion_event.delta,
        });
    }
}
