use crate::game_state::GameState;
use crate::systems::assets;
use bevy::prelude::*;

// Use <HandleUntyped> to refer to any asset, regardless of the asset type and allow to store a
// collection containing assets of mixed types.
#[derive(Resource, Default)]
pub struct LoadingAssets(pub Vec<HandleUntyped>);

pub struct AssetsReadyCheckerPlugin;

impl Plugin for AssetsReadyCheckerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingAssets>().add_systems(
            Update,
            (assets::check_load_state).run_if(in_state(GameState::LoadingAssets)),
        );
    }
}
