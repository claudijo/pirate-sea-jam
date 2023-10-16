use crate::components::button::StartButton;
use crate::components::layout::StartMenuLayout;
use crate::game_state::GameState;
use crate::resources::assets::FontAssets;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.96, 0.49, 0.18);
const HOVERED_BUTTON: Color = Color::rgb(0.94, 0.42, 0.18);

pub fn setup_start_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            StartMenuLayout,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
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
                        "Start Game",
                        TextStyle {
                            font: font_assets.font_handles["the-bomb-regular.otf"].clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn tear_down_start_menu(
    mut commands: Commands,
    entities: Query<Entity, With<StartMenuLayout>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_start_menu(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::InGame);
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
