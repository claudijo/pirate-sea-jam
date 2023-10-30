use crate::game_state::GameState;
use crate::resources::despawn::ShootingTargetDespawnEntities;
use crate::systems::shooting_target;
use bevy::prelude::*;

pub struct ShootingTargetPlugin;

impl Plugin for ShootingTargetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShootingTargetDespawnEntities::default())
            .add_systems(
                OnEnter(GameState::SplashScreen),
                shooting_target::spawn_shooting_target,
            )
            .add_systems(
                Update,
                (shooting_target::reset_shooting_target).run_if(in_state(GameState::InGame)),
            );
    }
}
