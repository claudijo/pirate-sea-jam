use crate::game_state::GameState;
use crate::plugins::assets::LoadingAssets;
use crate::resources::assets::ModelAssets;
use bevy::asset::LoadState;
use bevy::prelude::*;
use std::collections::HashMap;

const ASSET_NAMES: [&str; 11] = [
    "medium_flag",
    "medium_helm",
    "medium_hull",
    "medium_pirate_sail",
    "port_back_canon",
    "port_front_canon",
    "starboard_back_canon",
    "starboard_front_canon",
    "pirate_flag",
    "raft_with_mast",
    "cannon_ball",
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
    commands.insert_resource(ModelAssets { scene_handles });
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
            error!("Asset loading error");
        }
        _ => {} // NotLoaded / Loading / Unloaded
    };
}
