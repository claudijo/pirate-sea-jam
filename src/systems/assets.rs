use crate::plugins::assets_ready_checker::LoadingAssets;
use std::collections::HashMap;
use bevy::asset::LoadState;
use bevy::prelude::*;
use crate::game_state::GameState;
use crate::resources::assets::ShipAssets;

const ASSET_NAMES: [&str; 8] = [
    "medium_flag",
    "medium_helm",
    "medium_hull",
    "medium_pirate_sail",
    "port_back_canon",
    "port_front_canon",
    "starboard_back_canon",
    "starboard_front_canon",
];

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let mut scene_handles = HashMap::new();
    for name in ASSET_NAMES {
        let handle = asset_server.load(format!("models/{name}.glb#Scene0"));
        loading_assets.0.push(handle.clone_untyped());
        scene_handles.insert(name, handle);
    }
    commands.insert_resource(ShipAssets { scene_handles });
}

pub fn check_load_state(
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