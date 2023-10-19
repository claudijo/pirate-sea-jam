use crate::components::button::{ResetGameButton, StartGameButton};
use crate::components::layout::StartMenuLayout;
use crate::game_state::GameState;
use crate::resources::assets::FontAssets;
use crate::resources::player::InputDevice;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;

const START_BUTTON_NORMAL: Color = Color::rgb(0.9, 0.45, 0.21);
const START_BUTTON_HOVER: Color = Color::rgb(0.87, 0.36, 0.18);
// const RESTART_BUTTON_NORMAL: Color =  Color::rgb(0.05, 0.51, 0.56);
const RESTART_BUTTON_NORMAL: Color =  Color::rgb(0.31, 0.69, 0.32);
const RESTART_BUTTON_HOVER: Color = Color::rgb(0., 0.61, 0.45);


// Start game button mainly used for determining input device as well as focusing canvas element when
// loaded in browser
pub fn spawn_main_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.insert_resource(InputDevice::default());

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
                    StartGameButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(320.0),
                            height: Val::Px(64.0),
                            border: UiRect::all(Val::Px(4.0)),
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
    entities: Query<Entity, With<StartMenuLayout>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_main_menu_interactions(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartGameButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut touch_events: EventReader<TouchInput>,
    mut device: ResMut<InputDevice>,
) {
    for (interaction, mut background_color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                for event in touch_events.iter() {
                    if event.phase == TouchPhase::Started {
                        *device = InputDevice::Touch;
                    }
                }

                next_state.set(GameState::InGame);
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

pub fn spawn_restart_game_button(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            ResetGameButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(2.0)),
                    top: Val::Px(16.),
                    right: Val::Px(16.),
                    position_type: PositionType::Absolute,
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::WHITE),
                background_color: RESTART_BUTTON_NORMAL.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font: font_assets.font_handles["the-bomb-regular.otf"].clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}

pub fn handler_restart_game_button_interactions(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetGameButton>),
    >,
    mut touch_events: EventReader<TouchInput>,
) {
    for (interaction, mut background_color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {

            }
            Interaction::Hovered => {
                *background_color = RESTART_BUTTON_HOVER.into();
            }
            Interaction::None => {
                *background_color = RESTART_BUTTON_NORMAL.into();
            }
        }
    }
}
