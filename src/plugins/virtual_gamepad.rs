use crate::components::button::ReleasableTouchButton;
use crate::events::button::ButtonReleased;
use crate::game_state::GameState;
use crate::libs::plugins::virtual_joystick::{Joystick};
use crate::resources::player::InputDevice;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

pub const PLAYER_SHIP_STEERING_JOYSTICK: u8 = 0;
pub const CAMERA_JOYSTICK: u8 = 1;

const GAMEPAD_BUTTON_SIZE: f32 = 56.;

#[derive(Component)]
pub struct VirtualGamepadButton {
    background_color_normal: Color,
    background_color_pressed: Color,
    id: ButtonId,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonId {
    East,
    South,
}

#[derive(Event)]
pub struct GamepadButtonPressed {
    pub id: ButtonId,
}

#[derive(Event)]
pub struct GamepadButtonReleased {
    pub id: ButtonId,
}

const BUTTON_BORDER_NORMAL: Color = Color::rgba(1., 1., 1., 0.6);
const BUTTON_BORDER_PRESSED: Color = Color::rgb(1., 1., 1.);
const SOUTH_BUTTON_NORMAL: Color = Color::rgba(0.49, 0.70, 0.91, 0.6);
const SOUTH_BUTTON_PRESSED: Color = Color::rgb(0.49, 0.70, 0.91);
const EAST_BUTTON_NORMAL: Color = Color::rgba(1., 0.4, 0.4, 0.6);
const EAST_BUTTON_PRESSED: Color = Color::rgb(1., 0.4, 0.4);

fn spawn_left_stick(mut commands: Commands) {
    commands.spawn((
        Joystick::with_id(PLAYER_SHIP_STEERING_JOYSTICK),
        NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_right_stick(mut commands: Commands) {
    commands.spawn((
        Joystick::with_id(CAMERA_JOYSTICK).hide(),
        NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                right: Val::Px(0.),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_south_button(mut commands: Commands) {
    commands.spawn((
        VirtualGamepadButton {
            background_color_normal: SOUTH_BUTTON_NORMAL,
            background_color_pressed: SOUTH_BUTTON_PRESSED,
            id: ButtonId::South,
        },
        ReleasableTouchButton::default(),
        ButtonBundle {
            style: Style {
                width: Val::Px(GAMEPAD_BUTTON_SIZE),
                height: Val::Px(GAMEPAD_BUTTON_SIZE),
                border: UiRect::all(Val::Px(6.0)),
                bottom: Val::Px(32.),
                right: Val::Px(128.),
                position_type: PositionType::Absolute,
                ..default()
            },
            border_color: BorderColor(BUTTON_BORDER_NORMAL),
            background_color: SOUTH_BUTTON_NORMAL.into(),
            ..default()
        },
    ));
}

fn spawn_east_button(mut commands: Commands) {
    commands.spawn((
        VirtualGamepadButton {
            background_color_normal: EAST_BUTTON_NORMAL,
            background_color_pressed: EAST_BUTTON_PRESSED,
            id: ButtonId::East,
        },
        ReleasableTouchButton::default(),
        ButtonBundle {
            style: Style {
                width: Val::Px(GAMEPAD_BUTTON_SIZE),
                height: Val::Px(GAMEPAD_BUTTON_SIZE),
                border: UiRect::all(Val::Px(6.0)),
                bottom: Val::Px(128.),
                right: Val::Px(32.),
                position_type: PositionType::Absolute,
                ..default()
            },
            border_color: BorderColor(BUTTON_BORDER_NORMAL),
            background_color: EAST_BUTTON_NORMAL.into(),
            ..default()
        },
    ));
}

fn handle_gamepad_button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &VirtualGamepadButton,
        ),
        Changed<Interaction>,
    >,
    mut gamepad_button_pressed_event_writer: EventWriter<GamepadButtonPressed>,
) {
    for (interaction, mut background_color, mut border_color, virtual_gamepad_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                *background_color = virtual_gamepad_button.background_color_pressed.into();
                border_color.0 = BUTTON_BORDER_PRESSED;

                gamepad_button_pressed_event_writer.send(GamepadButtonPressed {
                    id: virtual_gamepad_button.id,
                })
            }

            Interaction::None => {
                *background_color = virtual_gamepad_button.background_color_normal.into();
                border_color.0 = BUTTON_BORDER_NORMAL;
            }

            Interaction::Hovered => {}
        }
    }
}

pub fn handle_gamepad_button_release(
    mut button_release_event_reader: EventReader<ButtonReleased>,
    mut gamepad_button_released_event_writer: EventWriter<GamepadButtonReleased>,
    virtual_gamepad_button_query: Query<&VirtualGamepadButton>,
) {
    for button_released in button_release_event_reader.iter() {
        if let Ok(virtual_gamepad_button) = virtual_gamepad_button_query.get(**button_released) {
            gamepad_button_released_event_writer.send(GamepadButtonReleased {
                id: virtual_gamepad_button.id,
            })
        }
    }
}

pub struct VirtualGamepadPlugin;

impl Plugin for VirtualGamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GamepadButtonPressed>();
        app.add_event::<GamepadButtonReleased>();

        app.add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_left_stick,
                spawn_right_stick,
                spawn_south_button,
                spawn_east_button,
            )
                .run_if(resource_exists_and_equals(InputDevice::Touch)),
        );

        app.add_systems(
            Update,
            (
                handle_gamepad_button_interaction,
                handle_gamepad_button_release,
            )
                .run_if(resource_exists_and_equals(InputDevice::Touch))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
