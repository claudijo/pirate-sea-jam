use bevy::prelude::*;

#[derive(Component)]
pub struct DebugText;

#[allow(dead_code)]
pub fn spawn_debug_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "...",
            TextStyle {
                font_size: 16.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        DebugText,
    ));
}

// Cheat sheet:
// mut debug_text: Query<&mut Text, With<DebugText>>,
// debug_text.single_mut().sections[0].value = output;
// app.add_systems(Startup, (crate::plugins::debug_text::spawn_debug_text));
