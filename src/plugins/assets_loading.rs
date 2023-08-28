use crate::game_state::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Assets(Vec<HandleUntyped>);

pub struct AssetsLoadingPlugin;

impl Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assets>()
            .add_systems(OnEnter(GameState::Loading), setup)
            .add_systems(
                Update,
                (check_assets_ready).run_if(in_state(GameState::Loading)),
            );
    }
}

fn setup(server: Res<AssetServer>, mut loading: ResMut<Assets>) {
    // we can have different asset types
    let boat_scene: Handle<Scene> = server.load("models/boat.glb#Scene0");

    // add them all to our collection for tracking
    loading.0.push(boat_scene.clone_untyped());
}

fn check_assets_ready(
    asset_server: Res<AssetServer>,
    assets: Res<Assets>,
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
