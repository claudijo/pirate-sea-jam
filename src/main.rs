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

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Pirate Sea Jam".into(),
            // This requires css html, body {margin: 0;height: 100%;} as explained https://github.com/bevyengine/bevy/pull/4726
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }));

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
        plugins::keyboard::KeyboardPlugin,
        plugins::wind::WindPlugin,
        plugins::text::TextOverlayPlugin,
        plugins::artillery::ArsenalPlugin,
        plugins::virtual_gamepad::VirtualGamepadPlugin,
    ));

    app.add_state::<game_state::GameState>();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run();
}
