use crate::debug_fps::resources::DebugFps;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

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

pub fn update_debug_fps(
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
