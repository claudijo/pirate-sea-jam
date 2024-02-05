pub mod resources;
mod systems;

use crate::assets::systems::{add_assets, check_assets_ready};
use crate::game_state::states::GameState;
use bevy::prelude::*;

const MODEL_FILE_NAMES: [&str; 7] = [
    "medium_flag.glb",
    "medium_helm.glb",
    "medium_hull.glb",
    "medium_pirate_sail.glb",
    "medium_canon.glb",
    "raft_with_mast.glb",
    "cannon_ball.glb",
];

const MESH_FILE_NAMES: [&str; 1] = ["medium_flag.glb"];

const FONT_FILE_NAMES: [&str; 1] = ["the-bomb-regular.otf"];

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadingAssets), add_assets);

        app.add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::LoadingAssets)),
        );
    }
}
