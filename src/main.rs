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
        plugins::assets::AssetsPlugin,
        plugins::camera::CameraPlugin,
        plugins::menu::StartMenuPlugin,
        plugins::ocean::OceanPlugin,
        plugins::light::LightPlugin,
        plugins::ship::ShipPlugin,
        plugins::shooting_target::ShootingTargetPlugin,
        plugins::pontoon::PontoonPlugin,
        plugins::keyboard_controller::KeyboadControllerPlugin,
        plugins::wind::WindPlugin,
        plugins::text::TextOverlayPlugin,
        plugins::artillery::ArsenalPlugin,
    ))
    .add_state::<game_state::GameState>();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run();
}
