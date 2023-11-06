use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use std::cmp::Ordering;
use std::collections::HashMap;

const TOUCH_MARKER_SIZE: f32 = 48.;
const ANCHOR_MARKER_SIZE: f32 = 24.;
const TRAIL_MARKER_SIZE: f32 = 16.;
const TRAIL_MARKERS_MIN_SPACING: f32 = 16.;
const BUTTON_SIZE: f32 = 48.;
const BUTTON_BORDER_NORMAL: Color = Color::rgba(1., 1., 1., 0.6);
const BUTTON_BORDER_PRESSED: Color = Color::rgb(1., 1., 1.);
const CROSS_BUTTON_NORMAL: Color = Color::rgba(0.49, 0.70, 0.91, 0.6);
const CROSS_BUTTON_PRESSED: Color = Color::rgb(0.49, 0.70, 0.91);
const CIRCLE_BUTTON_NORMAL: Color = Color::rgba(1., 0.4, 0.4, 0.6);
const CIRCLE_BUTTON_PRESSED: Color = Color::rgb(1., 0.4, 0.4);

#[derive(Event)]
pub struct VirtualJoystickMotion {
    pub delta: Vec2,

    // The identifier of the joystick
    pub id: u8,
}

#[derive(Component, Debug)]
pub struct Joystick {
    id: u8,
    touch_id: Option<u64>,
    is_hidden: bool,
    touch_position: Vec2,
    last_touch_position: Vec2,
    touch_start_position: Vec2,
}

impl Joystick {
    pub fn with_id(id: u8) -> Self {
        Self {
            id,
            touch_id: None,
            is_hidden: false,
            touch_position: Default::default(),
            last_touch_position: Default::default(),
            touch_start_position: Default::default(),
        }
    }

    pub fn hide(mut self) -> Self {
        self.is_hidden = true;
        self
    }
}

#[derive(Component)]
struct TouchMarker {
    touch_id: u64,
}

#[derive(Component)]
struct KnobMarker;

#[derive(Resource, Default)]
pub struct TrailMarkerEntities {
    pub by_touch_id: HashMap<u64, Vec<Entity>>,
}

pub fn trail_marker_spacing(anchor_knob_distance: f32) -> f32 {
    TRAIL_MARKERS_MIN_SPACING + anchor_knob_distance * 0.1
}

// Might end up with some unwanted touch markers in the UI if not clearing all touch events on
// entering the in game stage
fn clear_touch_events(mut touch_event_reader: EventReader<TouchInput>) {
    if !touch_event_reader.is_empty() {
        touch_event_reader.clear();
    }
}

fn spawn_knob_marker(commands: &mut Commands, position: Vec2, touch_id: u64) {
    commands.spawn((
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
        KnobMarker,
        TouchMarker { touch_id },
    ));
}

fn position_knob_marker(
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut knob_marker_query: Query<(&mut Style, &TouchMarker), With<KnobMarker>>,
) {
    for touch_input_event in touch_input_event_reader.iter() {
        if touch_input_event.phase != TouchPhase::Moved {
            continue;
        }

        for (mut style, touch_marker) in &mut knob_marker_query {
            if touch_marker.touch_id != touch_input_event.id {
                continue;
            }

            style.left = Val::Px(touch_input_event.position.x - TOUCH_MARKER_SIZE / 2.);
            style.top = Val::Px(touch_input_event.position.y - TOUCH_MARKER_SIZE / 2.);
        }
    }
}

fn spawn_anchor_marker(commands: &mut Commands, position: Vec2, touch_id: u64) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(ANCHOR_MARKER_SIZE),
                height: Val::Px(ANCHOR_MARKER_SIZE),
                left: Val::Px(position.x - ANCHOR_MARKER_SIZE / 2.),
                top: Val::Px(position.y - ANCHOR_MARKER_SIZE / 2.),
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

fn spawn_trail_marker(commands: &mut Commands, position: Vec2, touch_id: u64) -> Entity {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(1),
                style: Style {
                    width: Val::Px(TRAIL_MARKER_SIZE),
                    height: Val::Px(TRAIL_MARKER_SIZE),
                    left: Val::Px(position.x - TRAIL_MARKER_SIZE / 2.),
                    top: Val::Px(position.y - TRAIL_MARKER_SIZE / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::rgba(0.5, 0.5, 0.5, 0.2).into(),
                border_color: BorderColor(Color::rgba(1., 1., 1., 0.2)),
                ..default()
            },
            TouchMarker { touch_id },
        ))
        .id()
}

fn position_trail_markers(
    joystick_query: Query<&Joystick>,
    trail_marker_entities: Res<TrailMarkerEntities>,
    mut touch_marker_query: Query<&mut Style, With<TouchMarker>>,
) {
    for joystick in &joystick_query {
        if let Some(touch_id) = joystick.touch_id {
            if let Some(entities) = trail_marker_entities.by_touch_id.get(&touch_id) {
                let touch_drag_distance = joystick
                    .touch_start_position
                    .distance(joystick.touch_position);
                let dot_spacing = trail_marker_spacing(touch_drag_distance);
                let drag_inverse_vector =
                    joystick.touch_start_position - joystick.touch_position;

                let angle = drag_inverse_vector.y.atan2(drag_inverse_vector.x);
                let angle_sin = angle.sin();
                let angle_cos = angle.cos();

                for (i, entity) in entities.iter().enumerate() {
                    if let Ok(mut trail_dot_style) = touch_marker_query.get_mut(*entity) {
                        let magnitude = (i + 1) as f32 * dot_spacing;
                        let trail_dot_offset =
                            Vec2::new(magnitude * angle_cos, magnitude * angle_sin);
                        let trail_dot_position =
                            joystick.touch_position + trail_dot_offset;

                        trail_dot_style.left =
                            Val::Px(trail_dot_position.x - TRAIL_MARKER_SIZE / 2.);
                        trail_dot_style.top =
                            Val::Px(trail_dot_position.y - TRAIL_MARKER_SIZE / 2.);
                    }
                }
            }
        }
    }
}

fn generate_trail_markers(
    joystick_query: Query<(&Joystick)>,
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut commands: Commands,
    mut trail_marker_entities: ResMut<TrailMarkerEntities>,
) {
    for touch_input_event in touch_input_event_reader.iter() {
        if touch_input_event.phase != TouchPhase::Moved {
            continue;
        }

        for mut joystick in &joystick_query {
            if joystick.is_hidden {
                continue;
            }

            if let Some(touch_id) = joystick.touch_id {
                if touch_input_event.id != touch_id {
                    continue;
                }

                if let Some(entities) = trail_marker_entities.by_touch_id.get_mut(&touch_id) {
                    let touch_drag_distance = touch_input_event
                        .position
                        .distance(joystick.touch_start_position);
                    let drag_marker_spacing = trail_marker_spacing(touch_drag_distance);
                    let drag_markers_count = (touch_drag_distance / drag_marker_spacing) as usize;

                    match entities.len().cmp(&drag_markers_count) {
                        Ordering::Less => {
                            while entities.len() < drag_markers_count {
                                let entity = spawn_trail_marker(
                                    &mut commands,
                                    joystick.touch_start_position,
                                    touch_input_event.id,
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
}

fn handle_touch_start(
    mut commands: Commands,
    mut joystick_query: Query<(&RelativeCursorPosition, &mut Joystick)>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut trail_marker_entities: ResMut<TrailMarkerEntities>,
) {
    // Prevent handling touch starts on UI buttons
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            return;
        }
    }

    for (relative_cursor_position, mut joystick) in &mut joystick_query {
        if !relative_cursor_position.mouse_over() {
            continue;
        }

        for touch_input_event in touch_input_event_reader.iter() {
            if touch_input_event.phase == TouchPhase::Started {
                if joystick.touch_id.is_some() {
                    continue;
                }

                joystick.touch_id = Some(touch_input_event.id);
                joystick.touch_start_position = touch_input_event.position;
                joystick.touch_position =  touch_input_event.position;
                joystick.last_touch_position =  touch_input_event.position;

                if !joystick.is_hidden {
                    spawn_anchor_marker(&mut commands, touch_input_event.position, touch_input_event.id);
                    spawn_knob_marker(&mut commands, touch_input_event.position, touch_input_event.id);

                    trail_marker_entities
                        .by_touch_id
                        .insert(touch_input_event.id, Vec::new());
                }
            }
        }
    }
}

fn handle_touch_drag(
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut joystick_query: Query<&mut Joystick>,
    mut virtual_joystick_motion_event_writer: EventWriter<VirtualJoystickMotion>
) {
    for touch_input_event in touch_input_event_reader.iter() {
        if touch_input_event.phase != TouchPhase::Moved {
            continue;
        }

        for mut joystick in &mut joystick_query {
            if let Some(touch_id) = joystick.touch_id {
                if touch_input_event.id != touch_id {
                    continue;
                }

                joystick.last_touch_position = joystick.touch_position;
                joystick.touch_position = touch_input_event.position;

                virtual_joystick_motion_event_writer.send(VirtualJoystickMotion {
                    delta: joystick.touch_position - joystick.last_touch_position,
                    id: joystick.id,
                });
            }
        }
    }
}

fn handle_touch_end(
    mut commands: Commands,
    mut joystick_query: Query<&mut Joystick>,
    mut touch_input_event_reader: EventReader<TouchInput>,
    touch_marker_entities: Query<(Entity, &TouchMarker)>,
    mut trail_marker_entities: ResMut<TrailMarkerEntities>,
) {
    for mut joystick in &mut joystick_query {
        if let Some(touch_id) = joystick.touch_id {
            for touch_input_event in touch_input_event_reader.iter() {
                if touch_input_event.phase == TouchPhase::Ended || touch_input_event.phase == TouchPhase::Canceled {
                    if touch_id == touch_input_event.id {
                        trail_marker_entities.by_touch_id.remove(&touch_id);
                        joystick.touch_id = None;

                        for (entity, marker) in &touch_marker_entities {
                            if marker.touch_id == touch_input_event.id {
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
        app.insert_resource(TrailMarkerEntities::default());
        app.add_event::<VirtualJoystickMotion>();

        // app.add_systems(
        //     OnEnter(GameState::InGame),
        //     (spawn_left_stick, spawn_right_stick, clear_touch_events)
        //         .run_if(resource_exists_and_equals(InputDevice::Touch)),
        // );

        app.add_systems(
            Update,
            (
                handle_touch_start,
                handle_touch_drag,
                handle_touch_end,
                generate_trail_markers,
                position_trail_markers,
                position_knob_marker,
            )
        );
    }
}
