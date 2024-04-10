// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::args::ArgsPlugin;
use crate::connection::systems::RollbackConfig;
use crate::controls::components::{Controls};
use crate::game_state::states::GameState;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_ggrs::{GgrsApp, GgrsPlugin};

use crate::connection::FPS;
use crate::debug_fps::DebugFpsPlugin;
use crate::focal_point::FocalPointPlugin;
use crate::instructions::InstructionsPlugin;
use crate::menu::MenuPlugin;
use crate::orbiting_camera::OrbitingCameraPlugin;
use crate::physics::PhysicsPlugin;
use crate::sky_box::SkyBoxPlugin;
use crate::sync_test::SyncTestPlugin;
use crate::widget_debug::WidgetDebugPlugin;
use crate::wind::WindPlugin;

mod args;
mod artillery;
mod assets;
mod camera;
mod connection;
mod debug_fps;
mod controls;
mod focal_point;
mod game_state;
mod inputs;
mod instructions;
mod light;
mod menu;
mod ocean;
mod orbiting_camera;
mod physics;
mod player;
mod sky_box;
mod stats;
mod sync_test;
mod utils;
mod widget_debug;
mod wind;

fn main() {
    let mut app = App::new();

    app.insert_resource(AssetMetaCheck::Never);

    app.add_state::<GameState>();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Pirate Sea Jam".into(),
            // This requires css html, body {margin: 0;height: 100%;} as explained https://github.com/bevyengine/bevy/pull/4726
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }));
    app.add_plugins(GgrsPlugin::<RollbackConfig>::default());
    // define frequency of rollback game logic update
    app.set_rollback_schedule_fps(FPS);

    app.add_plugins(light::LightPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(ocean::OceanPlugin);
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(controls::ShipPlugin);
    app.add_plugins(assets::AssetsPlugin);
    app.add_plugins(inputs::InputsPlugin);
    app.add_plugins(connection::ConnectionPlugin);
    app.add_plugins(artillery::ArtilleryPlugin);
    app.add_plugins(ArgsPlugin);
    app.add_plugins(SyncTestPlugin);
    app.add_plugins(FocalPointPlugin);
    app.add_plugins(OrbitingCameraPlugin);
    app.add_plugins(WindPlugin);
    app.add_plugins(SkyBoxPlugin);
    app.add_plugins(DebugFpsPlugin);
    app.add_plugins(InstructionsPlugin);
    app.add_plugins(MenuPlugin);
    app.add_plugins(PhysicsPlugin);
    app.add_plugins(WidgetDebugPlugin);

    app.register_type::<Controls>();

    #[cfg(debug_assertions)]
    app.add_plugins(EditorPlugin::default());

    #[cfg(debug_assertions)]
    app.add_plugins(stats::StatsPlugin);

    app.run();
}
