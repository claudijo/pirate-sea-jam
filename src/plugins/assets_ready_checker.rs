use crate::game_state::GameState;
use bevy::asset::LoadState;
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
            (check_load_state).run_if(in_state(GameState::LoadingAssets)),
        );
    }
}

fn check_load_state(
    asset_server: Res<AssetServer>,
    assets: Res<LoadingAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match asset_server.get_group_load_state(assets.0.iter().map(|a| a.id())) {
        LoadState::Loaded => {
            next_state.set(GameState::InGame);
        }
        LoadState::Failed => {
            error!("asset loading error");
        }
        _ => {} // NotLoaded / Loading / Unloaded
    };
}
