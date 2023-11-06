use bevy::prelude::*;

#[derive(Component)]
pub struct DebugText;

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
