use crate::components::virtual_gamepad::{
    DebugText, GamepadTracker, TouchController, TouchMarker, TouchTrailMarker,
};
use crate::resources::virtual_gamepad::TouchTrailEntities;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use std::cmp::Ordering;

const GAMEPAD_ANCHOR_SIZE: f32 = 24.;
const GAMEPAD_TOUCH_SIZE: f32 = 48.;
const GAMEPAD_TRAIL_DOT_SIZE: f32 = 16.;
const MIN_DISTANCE_BETWEEN_TOUCH_TRAIL_MARKERS: f32 = 24.;

pub fn distance_between_dots(total_distance: f32) -> f32 {
    MIN_DISTANCE_BETWEEN_TOUCH_TRAIL_MARKERS + total_distance * 0.1
}

fn spawn_touch_trail_marker(
    commands: &mut Commands,
    touch_position: Vec2,
    touch_id: u64,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(GAMEPAD_TRAIL_DOT_SIZE),
                    height: Val::Px(GAMEPAD_TRAIL_DOT_SIZE),
                    left: Val::Px(touch_position.x - GAMEPAD_TRAIL_DOT_SIZE / 2.),
                    top: Val::Px(touch_position.y - GAMEPAD_TRAIL_DOT_SIZE / 2.),
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

fn spawn_touch_anchor_marker(
    commands: &mut Commands,
    touch_position: Vec2,
    touch_id: u64,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(GAMEPAD_ANCHOR_SIZE),
                    height: Val::Px(GAMEPAD_ANCHOR_SIZE),
                    left: Val::Px(touch_position.x - GAMEPAD_ANCHOR_SIZE / 2.),
                    top: Val::Px(touch_position.y - GAMEPAD_ANCHOR_SIZE / 2.),
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
                style: Style {
                    width: Val::Px(GAMEPAD_TOUCH_SIZE),
                    height: Val::Px(GAMEPAD_TOUCH_SIZE),
                    left: Val::Px(touch_position.x - GAMEPAD_TOUCH_SIZE / 2.),
                    top: Val::Px(touch_position.y - GAMEPAD_TOUCH_SIZE / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: Color::rgba(0.5, 0.5, 0.5, 0.8).into(),
                border_color: BorderColor(Color::rgba(1., 1., 1., 0.8)),
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

pub fn spawn_reset_button() {

}

pub fn spawn_fire_button() {}


pub fn init_movement_gamepad(mut commands: Commands, mut texts: Query<&mut Text, With<DebugText>>) {
    for mut text in &mut texts {
        text.sections[0].value = "Init movement gamepad".to_string();
    }
    commands.insert_resource(TouchTrailEntities::default());
    commands.spawn(GamepadTracker::default());
}

pub fn capture_virtual_gamepad(
    mut commands: Commands,
    mut touch_events: EventReader<TouchInput>,
    mut gamepad_trackers: Query<&mut GamepadTracker>,
    mut touch_trail_entities: ResMut<TouchTrailEntities>,
) {
    for event in touch_events.iter() {
        if event.phase == TouchPhase::Started {
            for mut tracker in &mut gamepad_trackers {
                if tracker.touch_id.is_some() {
                    continue;
                }

                tracker.touch_id = Some(event.id);

                spawn_touch_anchor_marker(&mut commands, event.position, event.id);
                spawn_touch_marker(&mut commands, event.position, event.id);

                touch_trail_entities
                    .for_touch_id
                    .insert(event.id, Vec::new());
            }
        }
    }
}

pub fn track_virtual_gamepad(
    gamepad_trackers: Query<&GamepadTracker>,
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
                        style.left = Val::Px(event.position.x - GAMEPAD_TOUCH_SIZE / 2.);
                        style.top = Val::Px(event.position.y - GAMEPAD_TOUCH_SIZE / 2.);

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
                                        let entity = spawn_touch_trail_marker(
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

pub fn release_virtual_gamepad(
    mut commands: Commands,
    mut gamepad_trackers: Query<&mut GamepadTracker>,
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
    gamepad_trackers: Query<&GamepadTracker>,
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
                                Val::Px(trail_dot_position.x - GAMEPAD_TRAIL_DOT_SIZE / 2.);
                            trail_dot_style.top =
                                Val::Px(trail_dot_position.y - GAMEPAD_TRAIL_DOT_SIZE / 2.);
                        }
                    }
                }
            }
        }
    }
}
