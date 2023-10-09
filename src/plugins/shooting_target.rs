use crate::game_state::GameState;
use crate::resources::despawn::ShootingTargetDespawnEntities;
use crate::systems::movement;
use crate::systems::shooting_target;
use bevy::prelude::*;

pub struct ShootingTargetPlugin;

impl Plugin for ShootingTargetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShootingTargetDespawnEntities::default())
            .add_systems(
                OnEnter(GameState::InGame),
                shooting_target::spawn_shooting_target,
            );
        // .add_systems(
        //     Update,
        //     (
        //         movement::push_ship,
        //         movement::turn_ship,
        //         movement::rotate_helm,
        //         movement::flutter_masthead_pennant,
        //         movement::flutter_sails,
        //     )
        //         .run_if(in_state(GameState::InGame)),
        // );
    }
}
