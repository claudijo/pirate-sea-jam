use crate::components::cannon::Cannon;
use crate::components::ship::{Booster, Ship, TurnRate};
use crate::components::shooting_target::ShootingTarget;
use crate::resources::assets::ModelAssets;
use crate::resources::despawn::ShipDespawnEntities;
use crate::systems::ship::spawn_ship;
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
    let active = keys.just_pressed(KeyCode::ShiftLeft);

    for mut boosters in &mut boosters {
        boosters.active = active;
    }
}

pub fn fire_canons_at_nearest_target_using_keyboard(
    keys: Res<Input<KeyCode>>,
    ships: Query<&Transform, With<Ship>>,
    targets: Query<&Transform, With<ShootingTarget>>,
    mut cannons: Query<(&Transform, &mut Cannon)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for ship_transform in &ships {
            let mut closest_target: Option<(Vec3, f32)> = None;
            for target_transform in &targets {
                let distance = ship_transform
                    .translation
                    .distance(target_transform.translation);
                if let Some((_, closest_distance)) = closest_target {
                    if closest_distance > distance {
                        closest_target = Some((target_transform.translation, distance));
                    }
                } else {
                    closest_target = Some((target_transform.translation, distance));
                }
            }

            if let Some((closest_translation, _)) = closest_target {
                for (cannon_transform, mut cannon) in &mut cannons {
                    println!("Closest cannon {:?}", cannon.is_shooting);
                    cannon.is_shooting = cannon_transform.translation.dot(closest_translation) > 0.;
                }
            }
        }
    } else {
        for (_, mut cannon) in &mut cannons {
            cannon.is_shooting = false;
        }
    }
}

// Temporary escape hatch so that the player can restart the game if ship is lost
pub fn reset_game(
    mut commands: Commands,
    ship_assets: Res<ModelAssets>,
    mut ship_despawn: ResMut<ShipDespawnEntities>,
    ships: Query<Entity, With<Ship>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::R) {
        // Note that some joint related child entities seem to be missing from the normal
        // parent-child-hierarchy when despawning, so those are registered and handled "manually".
        // (See https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier3d/examples/joints_despawn3.rs)
        for parent in &ships {
            for entity in &ship_despawn.entities {
                commands.entity(*entity).despawn();
            }

            ship_despawn.entities.clear();

            commands.entity(parent).despawn_recursive();
        }

        // Spawn new ship
        spawn_ship(commands, ship_assets, ship_despawn);
    }
}
