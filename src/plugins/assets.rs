use crate::game_state::GameState;
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

// #[derive(Resource, Default)]
// struct LoadedFolderHandles {
//     model_folder: Handle<LoadedFolder>,
//     font_folder: Handle<LoadedFolder>,
// }

#[derive(Resource)]
pub struct ModelAssets {
    pub scene_handles: HashMap<&'static str, Handle<Scene>>,
}

#[derive(Resource)]
pub struct FontAssets {
    pub font_handles: HashMap<&'static str, Handle<Font>>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut loaded_folder_handles: ResMut<LoadedFolderHandles>,
) {
    // This does not quite work. Reports that 'asset paths must have extensions'. Might be a bug. Keep looking for correct solution.
    // loaded_folder_handles.model_folder = asset_server.load_folder("models");
    // loaded_folder_handles.font_folder = asset_server.load_folder("fonts");

    // If you want a handle to a specific asset in a loaded folder, the easiest way to get one is to call load.
    // It will _not_ be loaded a second time.
    let mut scene_handles = HashMap::new();
    let mut font_handles = HashMap::new();

    for name in MODEL_FILE_NAMES {
        let handle = asset_server.load(format!("models/{name}#Scene0"));
        scene_handles.insert(name, handle);
    }

    for name in FONT_FILE_NAMES {
        let handle = asset_server.load(format!("fonts/{name}"));
        font_handles.insert(name, handle);
    }

    commands.insert_resource(ModelAssets { scene_handles });
    commands.insert_resource(FontAssets { font_handles });
}

fn check_assets_ready(
    model_assets: Res<ModelAssets>,
    font_assets: Res<FontAssets>,
    // loaded_folder_handles: Res<LoadedFolderHandles>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Does not work as it is dependent on loading folders. See above.
    // if asset_server.recursive_dependency_load_state(&loaded_folder_handles.model_folder)
    //     == RecursiveDependencyLoadState::Loaded
    //     && asset_server.recursive_dependency_load_state(&loaded_folder_handles.font_folder)
    //         == RecursiveDependencyLoadState::Loaded
    // {
    //     next_state.set(GameState::SplashScreen);
    // }

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

/**


fn resolve_loaded_untyped_handle(loading_handle: Res<LoadingUntypedHandle>, loaded_untyped_assets: Res<Assets<LoadedUntypedAsset>>) {
    if let Some(loaded_untyped_asset) = loaded_untyped_assets.get(&loading_handle.0) {
        let handle = loaded_untyped_asset.handle.clone();
        // continue working with `handle` which points to the asset at the originally requested path
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingUntypedHandles>,
) {
    let mut scene_handles = HashMap::new();
    let mut font_handles = HashMap::new();

    for name in MODEL_FILE_NAMES {
        let handle = asset_server.load_untyped(format!("models/{name}#Scene0"));
        loading_assets.0.push(handle.clone());
        scene_handles.insert(name, handle);
    }

    for name in FONT_FILE_NAMES {
        let handle = asset_server.load_untyped(format!("fonts/{name}"));
        loading_assets.0.push(handle.clone());
        font_handles.insert(name, handle);
    }

    commands.insert_resource(ModelAssets { scene_handles });
    commands.insert_resource(FontAssets { font_handles });
}

fn check_assets_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<LoadingUntypedHandles>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match asset_server.get_load_state(assets.0.iter().map(|a| a.id())) {
        None => {
            error!("Missing load state");
        }
        Some(load_state) => {
            match load_state {
                LoadState::Loaded => {
                    next_state.set(GameState::SplashScreen);

                    // remove the resource to drop the tracking handles
                    commands.remove_resource::<LoadingUntypedHandles>();
                    // (note: if you don't have any other handles to the assets
                    // elsewhere, they will get unloaded after this)
                }
                LoadState::Failed => {
                    error!("Failed asset load state");
                }
                _ => {} // NotLoaded / Loading / Unloaded
            }

        }
    };
}

**/
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<LoadedFolderHandles>();

        app.add_systems(OnEnter(GameState::LoadingAssets), setup);

        app.add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::LoadingAssets)),
        );
    }
}
