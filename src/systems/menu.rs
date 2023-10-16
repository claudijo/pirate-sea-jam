use crate::components::button::StartButton;
use crate::resources::assets::FontAssets;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.96, 0.49, 0.18);
const HOVERED_BUTTON: Color = Color::rgb(0.94, 0.42, 0.18);

pub fn update_start_menu(
    mut interactions: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<StartButton>)>,
) {
    for (interaction, mut background_color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
               // Start game
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_start_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    StartButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(360.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Anchors aweigh",
                        TextStyle {
                            font: font_assets.font_handles["the-bomb-regular.otf"].clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}
