use crate::components::ship::{PlayerShip, ShipBooster, ShipRudder};
use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::events::game::RestartGameEvent;
use crate::systems::movement::{RATE_OF_ROTATION, TURN_RATE_LIMIT};
use bevy::prelude::*;

pub fn handle_turning_keys_pressed(
    keys: Res<Input<KeyCode>>,
    mut rudder_query: Query<&mut ShipRudder>,
    time: Res<Time>,
) {
    for mut rudder in &mut rudder_query {
        if keys.pressed(KeyCode::A) {
            let new_angle = rudder.turn_rate - time.delta_seconds() * RATE_OF_ROTATION;
            rudder.turn_rate = new_angle.max(-TURN_RATE_LIMIT);
        }

        if keys.pressed(KeyCode::D) {
            let new_angle = rudder.turn_rate + time.delta_seconds() * RATE_OF_ROTATION;
            rudder.turn_rate = new_angle.min(TURN_RATE_LIMIT);
        }

        rudder.is_turning = keys.any_pressed([KeyCode::A, KeyCode::D]);
    }
}

pub fn handle_turbo_booster_key_pressed(
    keys: Res<Input<KeyCode>>,
    mut ship_query: Query<&mut ShipBooster, With<PlayerShip>>,
) {
    if keys.just_pressed(KeyCode::ShiftLeft) {
        for mut booster in &mut ship_query {
            booster.active = true;
        }
    }
}

pub fn handle_fire_key_pressed(
    ship_query: Query<Entity, With<PlayerShip>>,
    keys: Res<Input<KeyCode>>,
    mut aim_cannon_event_writer: EventWriter<AimCannonEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for entity in &ship_query {
            aim_cannon_event_writer.send(AimCannonEvent(entity));
        }
    }
}

pub fn handle_fire_key_released(
    ship_query: Query<Entity, With<PlayerShip>>,
    keys: Res<Input<KeyCode>>,
    mut fire_cannon_event_writer: EventWriter<FireCannonEvent>,
) {
    if keys.just_released(KeyCode::Space) {
        for entity in &ship_query {
            fire_cannon_event_writer.send(FireCannonEvent(entity));
        }
    }
}

pub fn handle_restart_game_key_pressed(
    keys: Res<Input<KeyCode>>,
    mut restart_game_event_writer: EventWriter<RestartGameEvent>,
) {
    if keys.just_pressed(KeyCode::R) {
        restart_game_event_writer.send(RestartGameEvent(Entity::PLACEHOLDER));
    }
}
