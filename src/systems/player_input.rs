use crate::components::ship::{Booster, Ship, TurnRate};
use bevy::prelude::*;

const RATE_OF_ROTATION: f32 = 1.5;
const TURN_RATE_LIMIT: f32 = 1.;

pub fn turn_ship_using_keyboard(
    keys: Res<Input<KeyCode>>,
    mut rate_of_turns: Query<&mut TurnRate>,
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

pub fn boost_ship_using_keyboard(keys: Res<Input<KeyCode>>, mut boosters: Query<&mut Booster>) {
    let active = if keys.just_pressed(KeyCode::ShiftLeft) {
        true
    } else {
        false
    };

    for mut boosters in &mut boosters {
        boosters.active = active;
    }
}

// Temporary escape hatch so that the player can restart the game if ship is lost
pub fn reset_game(keys: Res<Input<KeyCode>>, mut ships: Query<&mut Transform, With<Ship>>) {
    if keys.just_pressed(KeyCode::R) {
        for mut transform in &mut ships {
            transform.translation = Vec3::ZERO;
            transform.rotation = Quat::IDENTITY;
        }
    }
}
