// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;

mod utils;
mod resources;
mod components;
mod plugins;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            plugins::camera::CameraPlugin,
            plugins::ocean::OceanPlugin,
            plugins::light::LigthPlugin,
        ))
        .run();
}

