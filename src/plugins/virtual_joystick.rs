use std::cmp::Ordering;
use bevy::input::touch::TouchPhase;
use crate::game_state::GameState;
use crate::plugins::debug_text::{spawn_debug_text, DebugText};
use crate::resources::player::InputDevice;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use std::collections::HashMap;

const TOUCH_MARKER_SIZE: f32 = 48.;
const TOUCH_ANCHOR_SIZE: f32 = 24.;
const TOUCH_TRAIL_DOT_SIZE: f32 = 16.;
const MIN_DISTANCE_BETWEEN_TOUCH_TRAIL_MARKERS: f32 = 16.;
const GAMEPAD_BUTTON_SIZE: f32 = 48.;
const BUTTON_BORDER_NORMAL: Color = Color::rgba(1., 1., 1., 0.6);
const BUTTON_BORDER_PRESSED: Color = Color::rgb(1., 1., 1.);
const CROSS_BUTTON_NORMAL: Color = Color::rgba(0.49, 0.70, 0.91, 0.6);
const CROSS_BUTTON_PRESSED: Color = Color::rgb(0.49, 0.70, 0.91);
const CIRCLE_BUTTON_NORMAL: Color = Color::rgba(1., 0.4, 0.4, 0.6);
const CIRCLE_BUTTON_PRESSED: Color = Color::rgb(1., 0.4, 0.4);

#[derive(Default, Debug)]
enum JoystickId {
    #[default]
    Joystick1,
    Joystick2,
}

#[derive(Component, Debug, Default)]
struct Joystick {
    joystick_id: JoystickId,
    touch_id: Option<u64>,
    is_hidden: bool,
}

#[derive(Component)]
struct TouchMarker {
    touch_id: u64,
}

#[derive(Component)]
struct TouchDragMarker {
    touch_position: Vec2,
    last_touch_position: Vec2,
    touch_start_position: Vec2,
}

#[derive(Component)]
struct TouchTrailMarker;

#[derive(Resource, Default)]
pub struct TouchTrailMarkerEntities {
    pub for_touch_id: HashMap<u64, Vec<Entity>>,
}

pub fn distance_between_drag_markers(total_distance: f32) -> f32 {
    MIN_DISTANCE_BETWEEN_TOUCH_TRAIL_MARKERS + total_distance * 0.1
}

// Might end up with some unwanted touch markers in the UI if not clearing all touch events on
// entering the in game stage
fn clear_touch_events(
    mut touch_event_reader: EventReader<TouchInput>,
) {
    if !touch_event_reader.is_empty() {
        touch_event_reader.clear();
    }
}

fn spawn_left_stick(mut commands: Commands) {
    commands.spawn((
        Joystick { joystick_id: JoystickId::Joystick1, ..default() },
        RelativeCursorPosition::default(),
        NodeBundle {
            style: Style {
                width: Val::Percent(40.),
                height: Val::Percent(80.),

                top: Val::Px(0.),
                left: Val::Px(0.),

                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgba(235., 35., 12., 0.5).into(),
            ..default()
        },
    ));
}

fn spawn_right_stick(mut commands: Commands) {
    commands.spawn((
        Joystick { joystick_id: JoystickId::Joystick2, ..default() },
        RelativeCursorPosition::default(),
        NodeBundle {
            style: Style {
                width: Val::Percent(40.),
                height: Val::Percent(80.),

                top: Val::Px(0.),
                right: Val::Px(0.),

                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgba(235., 35., 12., 0.5).into(),
            ..default()
        },
    ));
}

fn spawn_touch_drag_marker(
    commands: &mut Commands, position: Vec2, touch_id: u64,
) {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(2),
                style: Style {
                    width: Val::Px(TOUCH_MARKER_SIZE),
                    height: Val::Px(TOUCH_MARKER_SIZE),
                    left: Val::Px(position.x - TOUCH_MARKER_SIZE / 2.),
                    top: Val::Px(position.y - TOUCH_MARKER_SIZE / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(6.0)),
                    ..default()
                },
                background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                border_color: BorderColor(Color::rgb(1., 1., 1.)),
                ..default()
            },
            TouchDragMarker {
                touch_position: position,
                last_touch_position: position,
                touch_start_position: position,
            },
            TouchMarker { touch_id },
        ));
}

fn spawn_touch_start_marker(
    commands: &mut Commands, position: Vec2, touch_id: u64,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(TOUCH_ANCHOR_SIZE),
                    height: Val::Px(TOUCH_ANCHOR_SIZE),
                    left: Val::Px(position.x - TOUCH_ANCHOR_SIZE / 2.),
                    top: Val::Px(position.y - TOUCH_ANCHOR_SIZE / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: Color::rgba(0.5, 0.5, 0.5, 0.4).into(),
                border_color: BorderColor(Color::rgba(1., 1., 1., 0.4)),
                ..default()
            },
            TouchMarker { touch_id },
        ));
}

fn spawn_touch_trail_marker(
    commands: &mut Commands,
    position: Vec2,
    touch_id: u64,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(1),
                style: Style {
                    width: Val::Px(TOUCH_TRAIL_DOT_SIZE),
                    height: Val::Px(TOUCH_TRAIL_DOT_SIZE),
                    left: Val::Px(position.x - TOUCH_TRAIL_DOT_SIZE / 2.),
                    top: Val::Px(position.y - TOUCH_TRAIL_DOT_SIZE / 2.),
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

fn arrange_touch_trail_markers(
    joystick_query: Query<&Joystick>,
    touch_drag_marker_query: Query<(&TouchDragMarker, &TouchMarker)>,
    touch_trail_marker_entities: Res<TouchTrailMarkerEntities>,
    mut touch_trail_markers: Query<&mut Style, (With<TouchTrailMarker>, Without<TouchDragMarker>)>,
) {
    for joystick in &joystick_query {
        if let Some(touch_id) = joystick.touch_id {
            for (touch_drag_marker, touch_marker) in &touch_drag_marker_query {
                if touch_marker.touch_id != touch_id {
                    continue;
                }

                if let Some(entities) = touch_trail_marker_entities.for_touch_id.get(&touch_id) {
                    let touch_drag_distance = touch_drag_marker
                        .touch_start_position
                        .distance(touch_drag_marker.touch_position);
                    let dot_spacing = distance_between_drag_markers(touch_drag_distance);
                    let drag_inverse_vector = touch_drag_marker.touch_start_position
                        - touch_drag_marker.touch_position;

                    let angle = drag_inverse_vector.y.atan2(drag_inverse_vector.x);
                    let angle_sin = angle.sin();
                    let angle_cos = angle.cos();

                    for (i, entity) in entities.iter().enumerate() {
                        if let Ok(mut trail_dot_style) = touch_trail_markers.get_mut(*entity) {
                            let magnitude = (i + 1) as f32 * dot_spacing;
                            let trail_dot_offset =
                                Vec2::new(magnitude * angle_cos, magnitude * angle_sin);
                            let trail_dot_position =
                                touch_drag_marker.touch_position + trail_dot_offset;

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

fn handle_touch_start(
    mut commands: Commands,
    mut relative_cursor_position_query: Query<(&RelativeCursorPosition, &mut Joystick)>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut touch_event_reader: EventReader<TouchInput>,
    mut touch_trail_entities: ResMut<TouchTrailMarkerEntities>,
    mut debug_text: Query<&mut Text, With<DebugText>>,
) {
    // Prevent handling touch starts on UI buttons
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            return;
        }
    }

    for (relative_cursor_position, mut joystick) in &mut relative_cursor_position_query {
        if !relative_cursor_position.mouse_over() {
            continue;
        }

        for touch_event in touch_event_reader.iter() {
            if touch_event.phase == TouchPhase::Started {
                if joystick.touch_id.is_some() {
                    continue;
                }

                joystick.touch_id = Some(touch_event.id);

                if !joystick.is_hidden {
                    spawn_touch_start_marker(&mut commands, touch_event.position, touch_event.id);
                    spawn_touch_drag_marker(&mut commands, touch_event.position, touch_event.id);

                    touch_trail_entities
                        .for_touch_id
                        .insert(touch_event.id, Vec::new());
                }
            }
        }
        // debug_text.single_mut().sections[0].value = output;
    }
}

fn handle_touch_drag(
    mut debug_text: Query<&mut Text, With<DebugText>>,
    mut commands: Commands,
    mut touch_event_reader: EventReader<TouchInput>,
    mut touch_drag_marker_query: Query<(&mut Style, &mut TouchDragMarker, &TouchMarker)>,
    mut touch_trail_entities: ResMut<TouchTrailMarkerEntities>,
) {
    for touch_event in touch_event_reader.iter() {
        if touch_event.phase != TouchPhase::Moved {
            continue;
        }

        for (mut style, mut touch_drag_marker, touch_marker) in &mut touch_drag_marker_query {
            if touch_marker.touch_id != touch_event.id {
                continue;
            }

            style.left = Val::Px(touch_event.position.x - TOUCH_MARKER_SIZE / 2.);
            style.top = Val::Px(touch_event.position.y - TOUCH_MARKER_SIZE / 2.);

            touch_drag_marker.last_touch_position = touch_drag_marker.touch_position;
            touch_drag_marker.touch_position = touch_event.position;

            // Add touch trail markers
            if let Some(entities) = touch_trail_entities.for_touch_id.get_mut(&touch_event.id) {
                let touch_drag_distance = touch_event.position.distance(touch_drag_marker.touch_start_position);
                let drag_marker_spacing = distance_between_drag_markers(touch_drag_distance);
                let drag_markers_count = (touch_drag_distance / drag_marker_spacing) as usize;

                debug_text.single_mut().sections[0].value = format!("drag_markers_count: {:?}", drag_markers_count);

                match entities.len().cmp(&drag_markers_count) {
                    Ordering::Less => {
                        while entities.len() < drag_markers_count {
                            let entity = spawn_touch_trail_marker(
                                &mut commands,
                                touch_drag_marker.touch_start_position,
                                touch_event.id,
                            );
                            entities.push(entity);
                        }
                    }
                    Ordering::Greater => {
                        while entities.len() > drag_markers_count {
                            if let Some(entity) = entities.pop() {
                                commands.entity(entity).despawn();
                            }
                        }
                    }
                    Ordering::Equal => {}
                }
            }
        }
    }
}

fn handle_touch_end(
    mut commands: Commands,
    mut joystick_query: Query<&mut Joystick>,
    mut touch_event_reader: EventReader<TouchInput>,
    touch_markers: Query<(Entity, &TouchMarker)>,
) {
    for mut joystick in &mut joystick_query {
        if let Some(touch_id) = joystick.touch_id {
            for event in touch_event_reader.iter() {
                if event.phase == TouchPhase::Ended || event.phase == TouchPhase::Canceled {
                    if touch_id == event.id {
                        joystick.touch_id = None;

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
}

pub struct VirtualJoystickPlugin;

impl Plugin for VirtualJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TouchTrailMarkerEntities::default());

        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_left_stick, spawn_right_stick, clear_touch_events)
                .run_if(resource_exists_and_equals(InputDevice::Touch)),
        );

        app.add_systems(Startup, (spawn_debug_text));

        app.add_systems(
            Update,
            (handle_touch_start, handle_touch_drag, handle_touch_end, arrange_touch_trail_markers)
                .run_if(resource_exists_and_equals(InputDevice::Touch))
                .run_if(in_state(GameState::InGame)),
        );
    }
}
