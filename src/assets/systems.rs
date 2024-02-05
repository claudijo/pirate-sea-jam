use crate::assets::resources::{FontAssets, ModelAssets};
use crate::assets::{FONT_FILE_NAMES, MESH_FILE_NAMES, MODEL_FILE_NAMES};
use crate::game_state::states::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn add_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut scene_handles = HashMap::new();
    let mut mesh_handles = HashMap::new();
    let mut font_handles = HashMap::new();

    for name in MODEL_FILE_NAMES {
        let handle = asset_server.load(format!("models/{name}#Scene0"));
        scene_handles.insert(name, handle);
    }

    for name in MESH_FILE_NAMES {
        let handle = asset_server.load(format!("models/{name}#Mesh0/Primitive0"));
        mesh_handles.insert(name, handle);
    }

    for name in FONT_FILE_NAMES {
        let handle = asset_server.load(format!("fonts/{name}"));
        font_handles.insert(name, handle);
    }

    commands.insert_resource(ModelAssets {
        scene_handles,
        mesh_handles,
    });

    commands.insert_resource(FontAssets { font_handles });
}

pub fn check_assets_ready(
    model_assets: Res<ModelAssets>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut all_loaded = true;

    for asset in model_assets.scene_handles.values() {
        if let Some(load_state) = asset_server.get_load_state(asset.id()) {
            if load_state != LoadState::Loaded {
                all_loaded = false;
                break;
            }
        } else {
            all_loaded = false;
            break;
        }
    }

    for asset in font_assets.font_handles.values() {
        if let Some(load_state) = asset_server.get_load_state(asset.id()) {
            if load_state != LoadState::Loaded {
                all_loaded = false;
                break;
            }
        } else {
            all_loaded = false;
            break;
        }
    }

    if all_loaded {
        next_state.set(GameState::SplashScreen);
    }
}
