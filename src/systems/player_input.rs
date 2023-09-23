use crate::components::ship::Ship;
use bevy::prelude::*;

const RATE_OF_ROTATION: f32 = 30.;
const TURN_RATE_LIMIT: f32 = 25.;

pub fn turn_ship_by_keyboard(
    keys: Res<Input<KeyCode>>,
    mut ships: Query<&mut Ship>,
    time: Res<Time>,
) {
    for mut ship in &mut ships {
        if keys.pressed(KeyCode::A) {
            let new_angle = ship.turn_rate - time.delta_seconds() * RATE_OF_ROTATION;
            ship.turn_rate = new_angle.max(-TURN_RATE_LIMIT);
        }

        if keys.pressed(KeyCode::D) {
            let new_angle = ship.turn_rate + time.delta_seconds() * RATE_OF_ROTATION;
            ship.turn_rate = new_angle.min(TURN_RATE_LIMIT);
        }

        let is_turning = keys.any_pressed([KeyCode::A, KeyCode::D]);

        // Return rudder to zero if not not turning
        if ship.turn_rate != 0. && !is_turning {
            if ship.turn_rate > 0. {
                let new_angle = ship.turn_rate - time.delta_seconds() * RATE_OF_ROTATION;
                ship.turn_rate = new_angle.max(0.);
            }

            if ship.turn_rate < 0. {
                let new_angle = ship.turn_rate + time.delta_seconds() * RATE_OF_ROTATION;
                ship.turn_rate = new_angle.min(0.);
            }
        }
    }
}
