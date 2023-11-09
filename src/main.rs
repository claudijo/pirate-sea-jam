// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::events::artillery::{AimCannonEvent, FireCannonEvent};
use crate::events::game::RestartGameEvent;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod components;
mod events;
mod game_state;
mod libs;
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
        plugins::mouse_input::MouseInputPlugin,
    ));

    app.add_plugins((
        plugins::virtual_gamepad_input::VirtualGamepadInputPlugin,
        plugins::orbiting_camera::OrbitingCameraPlugin,
        libs::plugins::virtual_joystick::VirtualJoystickPlugin,
        libs::plugins::touch_button::TouchButtonPlugin,
    ));

    app.add_state::<game_state::GameState>();

    app.add_event::<RestartGameEvent>();
    app.add_event::<FireCannonEvent>();
    app.add_event::<AimCannonEvent>();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run();
}
