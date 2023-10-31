use crate::components::ship::{PlayerId, Ship, ShipBooster, ShipTurnRate};
use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::events::game::RestartGameEvent;
use bevy::prelude::*;
use crate::systems::movement::TURN_RATE_LIMIT;

const RATE_OF_ROTATION: f32 = 1.5;


pub fn turn_ship(
    keys: Res<Input<KeyCode>>,
    mut rate_of_turns: Query<&mut ShipTurnRate>,
    time: Res<Time>,
) {
    for mut rate_of_turn in &mut rate_of_turns {
        if keys.pressed(KeyCode::A) {
            let new_angle = rate_of_turn.value - time.delta_seconds() * RATE_OF_ROTATION;
            rate_of_turn.value = new_angle.max(-TURN_RATE_LIMIT);
        }

        if keys.pressed(KeyCode::D) {
            let new_angle = rate_of_turn.value + time.delta_seconds() * RATE_OF_ROTATION;
            rate_of_turn.value = new_angle.min(TURN_RATE_LIMIT);
        }

        let is_turning = keys.any_pressed([KeyCode::A, KeyCode::D]);

        // Return rudder to zero if not not turning
        if rate_of_turn.value != 0. && !is_turning {
            if rate_of_turn.value > 0. {
                let new_angle = rate_of_turn.value - time.delta_seconds() * RATE_OF_ROTATION;
                rate_of_turn.value = new_angle.max(0.);
            }

            if rate_of_turn.value < 0. {
                let new_angle = rate_of_turn.value + time.delta_seconds() * RATE_OF_ROTATION;
                rate_of_turn.value = new_angle.min(0.);
            }
        }
    }
}

pub fn boost_ship(
    keys: Res<Input<KeyCode>>,
    mut ship_query: Query<(&mut ShipBooster, &Ship)>
) {
    if keys.just_pressed(KeyCode::ShiftLeft) {
        for (mut booster, ship) in &mut ship_query {
            if ship.player_id == PlayerId::PlayerOne {
                booster.active = true;
            }
        }
    }
}

pub fn handle_fire_key_pressed(
    ship_query: Query<(Entity, &Ship)>,
    keys: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AimCannonEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, ship) in &ship_query {
            if ship.player_id == PlayerId::PlayerOne {
                event_writer.send(AimCannonEvent(entity));
            }
        }
    }
}

pub fn handle_fire_key_released(
    ship_query: Query<(Entity, &Ship)>,
    keys: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<FireCannonEvent>,
) {
    if keys.just_released(KeyCode::Space) {
        for (entity, ship) in &ship_query {
            if ship.player_id == PlayerId::PlayerOne {
                event_writer.send(FireCannonEvent(entity));
            }
        }
    }
}

pub fn handle_restart_game_key_pressed(
    keys: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<RestartGameEvent>,
) {
    if keys.just_pressed(KeyCode::R) {
        event_writer.send(RestartGameEvent(Entity::PLACEHOLDER));
    }
}
