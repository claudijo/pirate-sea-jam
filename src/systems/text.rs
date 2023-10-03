use bevy::prelude::*;

pub fn display_text_overlay(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_section(
            "[A] turn to port | [D] turn to starboard | [Left shift] boost | [R] reset",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        }),
    ));
}
