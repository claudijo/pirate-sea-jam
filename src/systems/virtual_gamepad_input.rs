use crate::components::virtual_gamepad::{
    DebugText, JoystickTracker, TouchController, TouchMarker, TouchTrailMarker,
};
use crate::resources::virtual_gamepad::TouchTrailEntities;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use std::cmp::Ordering;
use crate::components::button::{CircleGamepadButton, CrossGamepadButton};

const TOUCH_MARKER_SIZE: f32 = 48.;
const TOUCH_ANCHOR_SIZE: f32 = 24.;
const TOUCH_TRAIL_DOT_SIZE: f32 = 16.;
const MIN_DISTANCE_BETWEEN_TOUCH_TRAIL_MARKERS: f32 = 16.;
const GAMEPAD_BUTTON_SIZE: f32 = 48.;
const BUTTON_BORDER_NORMAL: Color = Color::rgba(1., 1., 1., 0.6);
const CROSS_BUTTON_NORMAL: Color = Color::rgba(0.49, 0.70, 0.91, 0.6);
const CIRCLE_BUTTON_NORMAL: Color = Color::rgba(1., 0.4, 0.4, 0.6);

pub fn distance_between_dots(total_distance: f32) -> f32 {
    MIN_DISTANCE_BETWEEN_TOUCH_TRAIL_MARKERS + total_distance * 0.1
}

pub fn spawn_cross_button(
    mut commands: Commands,
) {
    commands
        .spawn((
            CrossGamepadButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(GAMEPAD_BUTTON_SIZE),
                    height: Val::Px(GAMEPAD_BUTTON_SIZE),
                    border: UiRect::all(Val::Px(6.0)),
                    bottom: Val::Px(32.),
                    right: Val::Px(96.),
                    position_type: PositionType::Absolute,
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(BUTTON_BORDER_NORMAL),
                background_color: CROSS_BUTTON_NORMAL.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "×",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

pub fn spawn_circle_button(
    mut commands: Commands,
) {
    commands
        .spawn((
            CircleGamepadButton,
            ButtonBundle {
                style: Style {
                    width: Val::Px(GAMEPAD_BUTTON_SIZE),
                    height: Val::Px(GAMEPAD_BUTTON_SIZE),
                    border: UiRect::all(Val::Px(6.0)),
                    bottom: Val::Px(96.),
                    right: Val::Px(32.),
                    position_type: PositionType::Absolute,
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(BUTTON_BORDER_NORMAL),
                background_color: CIRCLE_BUTTON_NORMAL.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "○",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

pub fn handle_cross_button_interactions(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CrossGamepadButton>),
    >,
) {
    for (interaction, mut background_color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                println!("cross button pressed");
            }
            Interaction::Hovered => {
                println!("cross button hovered");
            }
            Interaction::None => {
                println!("cross button none")
            }

        }
    }
}

fn spawn_touch_trail(
    commands: &mut Commands,
    touch_position: Vec2,
    touch_id: u64,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(1),
                style: Style {
                    width: Val::Px(TOUCH_TRAIL_DOT_SIZE),
                    height: Val::Px(TOUCH_TRAIL_DOT_SIZE),
                    left: Val::Px(touch_position.x - TOUCH_TRAIL_DOT_SIZE / 2.),
                    top: Val::Px(touch_position.y - TOUCH_TRAIL_DOT_SIZE / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::rgba(0.5, 0.5, 0.5, 0.2).into(),
                border_color: BorderColor(Color::rgba(1., 1., 1., 0.2)),
                ..default()
            },
            TouchMarker { touch_id },
            TouchTrailMarker,
        ))
        .id()
}

fn spawn_touch_anchor(
    commands: &mut Commands,
    touch_position: Vec2,
    touch_id: u64,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(TOUCH_ANCHOR_SIZE),
                    height: Val::Px(TOUCH_ANCHOR_SIZE),
                    left: Val::Px(touch_position.x - TOUCH_ANCHOR_SIZE / 2.),
                    top: Val::Px(touch_position.y - TOUCH_ANCHOR_SIZE / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: Color::rgba(0.5, 0.5, 0.5, 0.4).into(),
                border_color: BorderColor(Color::rgba(1., 1., 1., 0.4)),
                ..default()
            },
            TouchMarker { touch_id },
        ))
        .id()
}

fn spawn_touch_marker(commands: &mut Commands, touch_position: Vec2, touch_id: u64) -> Entity {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(2),
                style: Style {
                    width: Val::Px(TOUCH_MARKER_SIZE),
                    height: Val::Px(TOUCH_MARKER_SIZE),
                    left: Val::Px(touch_position.x - TOUCH_MARKER_SIZE / 2.),
                    top: Val::Px(touch_position.y - TOUCH_MARKER_SIZE / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
                background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                border_color: BorderColor(Color::rgb(1., 1., 1.)),
                ..default()
            },
            TouchController {
                start_position: touch_position,
                touch_position,
            },
            TouchMarker { touch_id },
        ))
        .id()
}

pub fn show_debug_text(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Debug...",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(64.0),
            left: Val::Px(16.0),
            ..default()
        }),
        DebugText,
    ));
}


pub fn init_movement_gamepad(mut commands: Commands, mut texts: Query<&mut Text, With<DebugText>>) {
    for mut text in &mut texts {
        text.sections[0].value = "Init movement gamepad".to_string();
    }
    commands.insert_resource(TouchTrailEntities::default());
    commands.spawn(JoystickTracker::default());
}

pub fn capture_virtual_joystick(
    mut commands: Commands,
    mut touch_events: EventReader<TouchInput>,
    mut gamepad_trackers: Query<&mut JoystickTracker>,
    mut touch_trail_entities: ResMut<TouchTrailEntities>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,

) {
    // Prevent anchoring joystick on buttons
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            return;
        }
    }

    for event in touch_events.iter() {
        if event.phase == TouchPhase::Started {
            for mut tracker in &mut gamepad_trackers {
                if tracker.touch_id.is_some() {
                    continue;
                }

                tracker.touch_id = Some(event.id);

                spawn_touch_anchor(&mut commands, event.position, event.id);
                spawn_touch_marker(&mut commands, event.position, event.id);

                touch_trail_entities
                    .for_touch_id
                    .insert(event.id, Vec::new());
            }
        }
    }
}

pub fn track_virtual_joystick(
    gamepad_trackers: Query<&JoystickTracker>,
    mut commands: Commands,
    mut touch_events: EventReader<TouchInput>,
    mut touch_controllers: Query<(&mut Style, &TouchMarker, &mut TouchController)>,
    mut touch_trail_entities: ResMut<TouchTrailEntities>,
) {
    for event in touch_events.iter() {
        if event.phase == TouchPhase::Moved {
            for tracker in &gamepad_trackers {
                if let Some(touch_id) = tracker.touch_id {
                    if touch_id != event.id {
                        continue;
                    }

                    for (mut style, marker, mut controller) in &mut touch_controllers {
                        if marker.touch_id != event.id {
                            continue;
                        }
                        style.left = Val::Px(event.position.x - TOUCH_MARKER_SIZE / 2.);
                        style.top = Val::Px(event.position.y - TOUCH_MARKER_SIZE / 2.);

                        controller.touch_position = event.position;

                        // Add touch trail markers
                        if let Some(entities) = touch_trail_entities.for_touch_id.get_mut(&event.id)
                        {
                            let touch_drag_distance =
                                event.position.distance(controller.start_position);
                            let dot_spacing = distance_between_dots(touch_drag_distance);
                            let num_of_dots = (touch_drag_distance / dot_spacing) as usize;

                            match entities.len().cmp(&num_of_dots) {
                                Ordering::Greater => {
                                    while entities.len() > num_of_dots {
                                        if let Some(entity) = entities.pop() {
                                            commands.entity(entity).despawn();
                                        }
                                    }
                                }
                                Ordering::Less => {
                                    while entities.len() < num_of_dots {
                                        let entity = spawn_touch_trail(
                                            &mut commands,
                                            controller.start_position,
                                            event.id,
                                        );
                                        entities.push(entity);
                                    }
                                }
                                Ordering::Equal => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn release_virtual_joystick(
    mut commands: Commands,
    mut gamepad_trackers: Query<&mut JoystickTracker>,
    mut touch_events: EventReader<TouchInput>,
    touch_markers: Query<(Entity, &TouchMarker)>,
) {
    for mut tracker in &mut gamepad_trackers {
        if let Some(touch_id) = tracker.touch_id {
            for event in touch_events.iter() {
                if event.phase == TouchPhase::Ended || event.phase == TouchPhase::Canceled {
                    if touch_id != event.id {
                        continue;
                    }

                    tracker.touch_id = None;

                    for (entity, marker) in &touch_markers {
                        if marker.touch_id == event.id {
                            commands.entity(entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

pub fn arrange_knob_trail_dots(
    gamepad_trackers: Query<&JoystickTracker>,
    touch_trail_entities: Res<TouchTrailEntities>,
    touch_controllers: Query<(&TouchMarker, &TouchController)>,
    mut touch_trail_markers: Query<&mut Style, (With<TouchTrailMarker>, Without<TouchController>)>,
) {
    for tracker in &gamepad_trackers {
        if let Some(touch_id) = tracker.touch_id {
            for (touch_marker_label, touch_marker_controller) in &touch_controllers {
                if touch_marker_label.touch_id != touch_id {
                    continue;
                }

                if let Some(entities) = touch_trail_entities.for_touch_id.get(&touch_id) {
                    let touch_drag_distance = touch_marker_controller
                        .start_position
                        .distance(touch_marker_controller.touch_position);
                    let dot_spacing = distance_between_dots(touch_drag_distance);
                    let drag_inverse_vector = touch_marker_controller.start_position
                        - touch_marker_controller.touch_position;

                    let angle = drag_inverse_vector.y.atan2(drag_inverse_vector.x);
                    let angle_sin = angle.sin();
                    let angle_cos = angle.cos();

                    for (i, entity) in entities.iter().enumerate() {
                        if let Ok(mut trail_dot_style) = touch_trail_markers.get_mut(*entity) {
                            let magnitude = (i + 1) as f32 * dot_spacing;
                            let trail_dot_offset =
                                Vec2::new(magnitude * angle_cos, magnitude * angle_sin);
                            let trail_dot_position =
                                touch_marker_controller.touch_position + trail_dot_offset;

                            trail_dot_style.left =
                                Val::Px(trail_dot_position.x - TOUCH_TRAIL_DOT_SIZE / 2.);
                            trail_dot_style.top =
                                Val::Px(trail_dot_position.y - TOUCH_TRAIL_DOT_SIZE / 2.);
                        }
                    }
                }
            }
        }
    }
}
