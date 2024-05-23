use bevy::prelude::*;
use crate::camera::resources::MainCamera;

pub fn display_control_keys(mut commands: Commands, main_camera: Res<MainCamera>,) {
    commands
        .spawn((
            // Seems to be required in dev builds since using editor plugin results in multiple
            // cameras
            TargetCamera(main_camera.id),
            NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Px(16.),
                    right: Val::Px(16.),
                    top: Val::Px(16.),
                    bottom: Val::Px(16.),
                },
                ..default()
            },
            ..default()
        },))
        .with_children(|child_builder| {
            child_builder.spawn(TextBundle::from_section(
                "[A] turn port | [D] turn starboard | [Space] fire cannons | [Mouse] orbit camera",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}
