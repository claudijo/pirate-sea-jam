// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod components;
mod game_state;
mod plugins;
mod resources;
mod systems;
mod utils;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins((
        plugins::assets_ready_checker::AssetsReadyCheckerPlugin,
        plugins::camera::CameraPlugin,
        plugins::ocean::OceanPlugin,
        plugins::light::LigthPlugin,
        plugins::ship::ShipPlugin,
        plugins::pontoon::PontoonPlugin,
        plugins::keyboad_controller::KeyboadControllerPlugin,
        plugins::wind::WindPlugin,
    ))
    .add_state::<game_state::GameState>();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run();
}
