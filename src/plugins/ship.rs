use crate::components::pontoon::Pontoon;
use crate::components::ship::{Booster, Helm, Pennant, Sail, Ship, TurnRate};
use crate::game_state::GameState;
use crate::systems::{movement, ship};
use bevy::prelude::*;
use crate::systems::assets;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadingAssets), assets::load_assets)
            .add_systems(OnEnter(GameState::InGame), ship::spawn_ship)
            .add_systems(
                Update,
                (
                    movement::push_ship,
                    movement::turn_ship,
                    movement::rotate_helm,
                    movement::flutter_masthead_pennant,
                    movement::flutter_sails,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}


