use crate::game_state::GameState;
use crate::plugins::assets::LoadingAssets;
use crate::resources::assets::{FontAssets, ModelAssets};
use bevy::asset::LoadState;
use bevy::prelude::*;
use std::collections::HashMap;

const MODEL_FILE_NAMES: [&str; 8] = [
    "medium_flag.glb",
    "medium_helm.glb",
    "medium_hull.glb",
    "medium_pirate_sail.glb",
    "medium_canon.glb",
    "pirate_flag.glb",
    "raft_with_mast.glb",
    "cannon_ball.glb",
];

const FONT_FILE_NAMES: [&str; 1] = ["the-bomb-regular.otf"];

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let mut scene_handles = HashMap::new();
    let mut font_handles = HashMap::new();

    for name in MODEL_FILE_NAMES {
        let handle = asset_server.load(format!("models/{name}#Scene0"));
        loading_assets.0.push(handle.clone_untyped());
        scene_handles.insert(name, handle);
    }

    for name in FONT_FILE_NAMES {
        let handle = asset_server.load(format!("fonts/{name}"));
        loading_assets.0.push(handle.clone_untyped());
        font_handles.insert(name, handle);
    }

    commands.insert_resource(ModelAssets { scene_handles });
    commands.insert_resource(FontAssets { font_handles });
}

pub fn check_load_state(
    asset_server: Res<AssetServer>,
    assets: Res<LoadingAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match asset_server.get_group_load_state(assets.0.iter().map(|a| a.id())) {
        LoadState::Loaded => {
            next_state.set(GameState::SplashScreen);
        }
        LoadState::Failed => {
            error!("Asset loading error");
        }
        _ => {} // NotLoaded / Loading / Unloaded
    };
}
