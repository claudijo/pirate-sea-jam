use crate::assets::resources::FontAssets;
use crate::camera::resources::MainCamera;
use crate::game_state::states::GameState;
use crate::menu::components::{StartGameButton, StartMenuLayout};
use crate::menu::{START_BUTTON_HOVER, START_BUTTON_NORMAL};
use bevy::prelude::*;

// Start game button mainly used for determining input device as well as focusing canvas element when
// loaded in browser
pub fn spawn_main_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    main_camera: Res<MainCamera>,
) {
    commands
        .spawn((
            // Seems to be required in dev builds since using editor plugin results in multiple
            // cameras
            TargetCamera(main_camera.id),
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
        .with_children(|child_builder| {
            child_builder
                .spawn((
                    StartGameButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(320.0),
                            height: Val::Px(64.0),
                            border: UiRect::all(Val::Px(8.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: START_BUTTON_NORMAL.into(),
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

pub fn despawn_main_menu(
    mut commands: Commands,
    start_menu_query: Query<Entity, With<StartMenuLayout>>,
) {
    for entity in &start_menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_main_menu_interactions(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartGameButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Matchmaking);
            }
            Interaction::Hovered => {
                *background_color = START_BUTTON_HOVER.into();
            }
            Interaction::None => {
                *background_color = START_BUTTON_NORMAL.into();
            }
        }
    }
}
