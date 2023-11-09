use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use std::cmp::Ordering;
use std::collections::HashMap;

const TOUCH_MARKER_SIZE: f32 = 56.;
const ANCHOR_MARKER_SIZE: f32 = 24.;
const TRAIL_MARKER_SIZE: f32 = 16.;
const TRAIL_MARKERS_MIN_SPACING: f32 = 16.;

#[derive(Bundle)]
pub struct VirtualJoystickBundle {
    pub focus_policy: FocusPolicy,
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,
}

impl Default for VirtualJoystickBundle {
    fn default() -> Self {
        Self {
            focus_policy: FocusPolicy::Block,
            node: Default::default(),
            style: Default::default(),
            background_color: Default::default(),
            border_color: BorderColor(Color::NONE),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

#[derive(Event)]
pub struct VirtualJoystickMotion {
    pub delta: Vec2,

    // The identifier of the joystick
    pub id: u8,
}

#[derive(Resource, Default)]
pub struct VirtualJoystickPosition {
    pub by_joystick_id: HashMap<u8, Vec2>,
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
struct TrailMarkerEntities {
    pub by_touch_id: HashMap<u64, Vec<Entity>>,
}

fn trail_marker_spacing(anchor_knob_distance: f32) -> f32 {
    TRAIL_MARKERS_MIN_SPACING + anchor_knob_distance * 0.1
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
                let drag_inverse_vector = joystick.touch_start_position - joystick.touch_position;

                let angle = drag_inverse_vector.y.atan2(drag_inverse_vector.x);
                let angle_sin = angle.sin();
                let angle_cos = angle.cos();

                for (i, entity) in entities.iter().enumerate() {
                    if let Ok(mut trail_dot_style) = touch_marker_query.get_mut(*entity) {
                        let magnitude = (i + 1) as f32 * dot_spacing;
                        let trail_dot_offset =
                            Vec2::new(magnitude * angle_cos, magnitude * angle_sin);
                        let trail_dot_position = joystick.touch_position + trail_dot_offset;

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
    joystick_query: Query<&Joystick>,
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut commands: Commands,
    mut trail_marker_entities: ResMut<TrailMarkerEntities>,
) {
    for touch_input_event in touch_input_event_reader.iter() {
        if touch_input_event.phase != TouchPhase::Moved {
            continue;
        }

        for joystick in &joystick_query {
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
    mut joystick_query: Query<(&Node, &GlobalTransform, &mut Joystick)>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut trail_marker_entities: ResMut<TrailMarkerEntities>,
    mut virtual_joystick_position: ResMut<VirtualJoystickPosition>,
) {
    // Prevent handling touch starts on UI buttons
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            return;
        }
    }

    for touch_input_event in touch_input_event_reader.iter() {
        if touch_input_event.phase != TouchPhase::Started {
            continue;
        }

        for (node, global_transform, mut joystick) in &mut joystick_query {
            let rect = node.logical_rect(global_transform);
            let is_inside = rect.contains(touch_input_event.position);

            if !is_inside {
                continue;
            }

            if joystick.touch_id.is_some() {
                continue;
            }

            joystick.touch_id = Some(touch_input_event.id);
            joystick.touch_start_position = touch_input_event.position;
            joystick.touch_position = touch_input_event.position;
            joystick.last_touch_position = touch_input_event.position;

            virtual_joystick_position
                .by_joystick_id
                .insert(joystick.id, Vec2::ZERO);

            if !joystick.is_hidden {
                spawn_anchor_marker(
                    &mut commands,
                    touch_input_event.position,
                    touch_input_event.id,
                );
                spawn_knob_marker(
                    &mut commands,
                    touch_input_event.position,
                    touch_input_event.id,
                );

                trail_marker_entities
                    .by_touch_id
                    .insert(touch_input_event.id, Vec::new());
            }
        }
    }
}

fn handle_touch_drag(
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut joystick_query: Query<&mut Joystick>,
    mut virtual_joystick_motion_event_writer: EventWriter<VirtualJoystickMotion>,
    mut virtual_joystick_position: ResMut<VirtualJoystickPosition>,
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

                virtual_joystick_position.by_joystick_id.insert(
                    joystick.id,
                    joystick.touch_position - joystick.touch_start_position,
                );

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
    mut trail_marker_entities: ResMut<TrailMarkerEntities>,
    mut virtual_joystick_position: ResMut<VirtualJoystickPosition>,
    touch_marker_entities: Query<(Entity, &TouchMarker)>,
) {
    for touch_input_event in touch_input_event_reader.iter() {
        if !(touch_input_event.phase == TouchPhase::Ended
            || touch_input_event.phase == TouchPhase::Canceled)
        {
            continue;
        }

        for mut joystick in &mut joystick_query {
            if let Some(touch_id) = joystick.touch_id {
                if touch_id != touch_input_event.id {
                    continue;
                }

                trail_marker_entities.by_touch_id.remove(&touch_id);
                virtual_joystick_position
                    .by_joystick_id
                    .remove(&joystick.id);
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

pub struct VirtualJoystickPlugin;

impl Plugin for VirtualJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrailMarkerEntities::default());
        app.insert_resource(VirtualJoystickPosition::default());

        app.add_event::<VirtualJoystickMotion>();

        app.add_systems(
            Update,
            (
                handle_touch_start,
                handle_touch_drag,
                handle_touch_end,
                generate_trail_markers,
                position_trail_markers,
                position_knob_marker,
            ),
        );
    }
}
