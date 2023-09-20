use crate::components::ship::Rudder;
use bevy::prelude::*;

const HELM_RATE_OF_ROTATION: f32 = 30.;

pub fn turn_ship_by_keyboard(
    keys: Res<Input<KeyCode>>,
    mut player_ships: Query<&mut Rudder>,
    time: Res<Time>,
) {
    for mut rudder in &mut player_ships {
        if keys.pressed(KeyCode::A) {
            let new_angle =
                rudder.angle - time.delta_seconds() * HELM_RATE_OF_ROTATION;
            rudder.angle = new_angle.max(-35.);
        }

        if keys.pressed(KeyCode::D) {
            let new_angle =
                rudder.angle + time.delta_seconds() * HELM_RATE_OF_ROTATION;
            rudder.angle = new_angle.min(35.);
        }

        let is_turning = keys.any_pressed([KeyCode::A, KeyCode::D]);

        // Return rudder to zero if not not turning
        if rudder.angle != 0. && !is_turning {
            if rudder.angle > 0. {
                let new_angle =
                    rudder.angle - time.delta_seconds() * HELM_RATE_OF_ROTATION;
                rudder.angle = new_angle.max(0.);
            }

            if rudder.angle < 0. {
                let new_angle =
                    rudder.angle + time.delta_seconds() * HELM_RATE_OF_ROTATION;
                rudder.angle = new_angle.min(0.);
            }
        }
    }
}
