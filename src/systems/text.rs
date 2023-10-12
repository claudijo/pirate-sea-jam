use bevy::prelude::*;

pub fn display_control_keys(mut commands: Commands) {
    commands.spawn((TextBundle::from_section(
        "[A] turn port | [D] turn starboard | [Space] fire | [Left shift] boost | [R] reset",
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
    }),));
}
