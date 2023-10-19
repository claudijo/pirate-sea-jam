use crate::game_state::GameState;
use crate::resources::despawn::ShipDespawnEntities;
use crate::systems::{movement, ship};
use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShipDespawnEntities::default())
            .add_systems(OnEnter(GameState::InGame), ship::spawn_ship)
            .add_systems(
                Update,
                (
                    (
                        movement::push_ship,
                        movement::turn_ship,
                        movement::rotate_helm,
                        movement::flutter_sails,
                        movement::flutter_pennant,

                        ship::reset_ship,
                    )
                        .run_if(in_state(GameState::InGame)),
                    movement::flutter_pennant.run_if(in_state(GameState::SplashScreen)),
                ),
            );
    }
}
