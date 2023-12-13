use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
pub struct DebugFps;

pub fn spawn_debug_fps(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ??",
            TextStyle {
                font_size: 14.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(2.),
            left: Val::Px(2.),
            ..default()
        }),
        DebugFps,
    ));
}

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<DebugFps>>,
) {
    for mut text in &mut fps_text_query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.average() {
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

pub struct DebugFpsPlugin;

impl Plugin for DebugFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);

        app.add_systems(Startup, spawn_debug_fps);

        app.add_systems(Update, update_fps);
    }
}
