use crate::components::cannon::{Cannon, CannonBarrelTilt, CannonCarriage, CannonGunPowder};
use crate::components::ship::{Ship, ShipBooster, ShipTurnRate};
use crate::components::shooting_target::ShootingTarget;
use crate::resources::assets::ModelAssets;
use crate::resources::despawn::{ShipDespawnEntities, ShootingTargetDespawnEntities};
use crate::systems::ship::spawn_ship;
use crate::systems::shooting_target::spawn_shooting_target;
use bevy::prelude::*;

const RATE_OF_ROTATION: f32 = 1.5;
const TURN_RATE_LIMIT: f32 = 1.;

pub fn turn_ship_using_keyboard(
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

pub fn boost_ship_using_keyboard(keys: Res<Input<KeyCode>>, mut boosters: Query<&mut ShipBooster>) {
    let active = keys.just_pressed(KeyCode::ShiftLeft);

    for mut boosters in &mut boosters {
        boosters.active = active;
    }
}

pub fn start_aiming_cannons_at_nearest_target_using_keyboard(
    keys: Res<Input<KeyCode>>,
    shooting_targets: Query<&Transform, With<ShootingTarget>>,
    ships: Query<(Entity, &Transform), With<Ship>>,
    mut cannons: Query<(&GlobalTransform, &Cannon, &mut CannonCarriage)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (ship_entity, ship_transform) in &ships {
            // Bail if there's already an aiming cannon on this ship
            for (_, cannon, cannon_carriage) in &cannons {
                if let Some(rig) = cannon.rig {
                    if rig == ship_entity && cannon_carriage.is_aiming {
                        return;
                    }
                }
            }

            let mut closest_target: Option<(Vec3, f32)> = None;
            for target_transform in &shooting_targets {
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
                for (cannon_global_transform, _, mut cannon_carriage) in &mut cannons {
                    let target_direction =
                        closest_translation - cannon_global_transform.translation();
                    cannon_carriage.is_aiming =
                        cannon_global_transform.left().dot(target_direction) > 0.;
                }
            }
        }
    }
}

pub fn tilt_aiming_cannons_using_keyboard(
    keys: Res<Input<KeyCode>>,
    mut cannons: Query<(
        &mut Transform,
        &mut CannonBarrelTilt,
        &CannonCarriage,
        &Cannon,
    )>,
    time: Res<Time>,
) {
    if keys.pressed(KeyCode::Space) {
        for (mut transform, mut barrel_tilt, carriage, cannon) in &mut cannons {
            if carriage.is_aiming {
                let angle = barrel_tilt.angle + time.delta_seconds() * cannon.tilt_factor;

                if cannon.tilt_factor > 0. {
                    barrel_tilt.angle = angle.min(cannon.max_tilt);
                } else {
                    barrel_tilt.angle = angle.max(-cannon.max_tilt);
                }
                transform.rotation =
                    Quat::from_rotation_z(cannon.default_tilt + barrel_tilt.angle.to_radians());
            }
        }
    }
}

pub fn fire_aiming_cannons_using_keyboard(
    keys: Res<Input<KeyCode>>,
    mut cannons: Query<(&mut CannonCarriage, &mut CannonGunPowder), With<Cannon>>,
) {
    if keys.just_released(KeyCode::Space) {
        for (mut carriage, mut gun_powder) in &mut cannons {
            if carriage.is_aiming {
                carriage.is_aiming = false;
                gun_powder.is_lit = true;
            }
        }
    }
}

// Temporary escape hatch so that the player can restart the game if ship is lost
pub fn reset_ship(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
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
        spawn_ship(commands, model_assets, ship_despawn);
    }
}

pub fn reset_shooting_target(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    mut shooting_target_despawn: ResMut<ShootingTargetDespawnEntities>,
    shooting_targets: Query<Entity, With<ShootingTarget>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::R) {
        for parent in &shooting_targets {
            for entity in &shooting_target_despawn.entities {
                commands.entity(*entity).despawn();
            }

            shooting_target_despawn.entities.clear();
            commands.entity(parent).despawn_recursive();
        }

        // Spawn new shooting target
        spawn_shooting_target(commands, model_assets, shooting_target_despawn);
    }
}
