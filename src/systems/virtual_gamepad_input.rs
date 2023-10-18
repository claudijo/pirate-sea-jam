use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct GamepadTracker {
    pub touch_id: Option<u64>,
    // TODO: Add GamepadInteractionArea
}

#[derive(Component, Default)]
pub struct GamepadInteractionArea {
    pub rect: Rect,
}

#[derive(Component)]
pub struct DebugText;

#[derive(Component)]
pub struct Controller {
    pub start_position: Vec2,
    // For convenient access to current touch position
    pub touch_position: Vec2,
}

#[derive(Component)]
pub struct Label {
    pub touch_id: u64,
}

#[derive(Resource, Default)]
pub struct KnobTrailEntities {
    pub for_touch_id: HashMap<u64, Vec<Entity>>,
}

#[derive(Component)]
pub struct KnobTrailDot;

pub fn distance_between_dots(min_distance: f32, total_distance: f32) -> f32 {
    min_distance + total_distance * 0.1
}

pub fn spawn_trail_dot(mut commands: Commands) {

}

const GAMEPAD_ANCHOR_SIZE: f32 = 32.;
const GAMEPAD_TOUCH_SIZE: f32 = 48.;

const GAMEPAD_TRAIL_DOT_SIZE: f32 = 16.;

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
        text.sections[0].value = format!("Init movement gamepad");
    }
    commands.insert_resource(KnobTrailEntities::default());
    commands.spawn((GamepadTracker::default()));
}

pub fn capture_virtual_gamepad(
    mut commands: Commands,
    mut touch_events: EventReader<TouchInput>,
    mut gamepad_trackers: Query<&mut GamepadTracker>,
    mut texts: Query<&mut Text, With<DebugText>>,
    mut knob_trail_entities: ResMut<KnobTrailEntities>,
) {
    for event in touch_events.iter() {
        match event.phase {
            TouchPhase::Started => {
                for mut tracker in &mut gamepad_trackers {
                    // if interaction_area.rect.contains(event.position) && tracker.finger_id.is_none()
                    if tracker.touch_id.is_none() {
                        for mut text in &mut texts {
                            text.sections[0].value = format!("capturing at {:?}", event.position);
                        }

                        tracker.touch_id = Some(event.id);

                        commands.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(GAMEPAD_ANCHOR_SIZE),
                                    height: Val::Px(GAMEPAD_ANCHOR_SIZE),
                                    left: Val::Px(event.position.x - GAMEPAD_ANCHOR_SIZE / 2.),
                                    top: Val::Px(event.position.y - GAMEPAD_ANCHOR_SIZE / 2.),
                                    position_type: PositionType::Absolute,
                                    border: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::GRAY.into(),
                                border_color: BorderColor(Color::WHITE),
                                ..default()
                            },
                            Label { touch_id: event.id },
                        ));

                        commands.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(GAMEPAD_TOUCH_SIZE),
                                    height: Val::Px(GAMEPAD_TOUCH_SIZE),
                                    left: Val::Px(event.position.x - GAMEPAD_TOUCH_SIZE / 2.),
                                    top: Val::Px(event.position.y - GAMEPAD_TOUCH_SIZE / 2.),
                                    position_type: PositionType::Absolute,
                                    border: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::SILVER.into(),
                                border_color: BorderColor(Color::WHITE),
                                ..default()
                            },
                            Controller {
                                start_position: event.position,
                                touch_position: event.position,
                            },
                            Label { touch_id: event.id },
                        ));

                        knob_trail_entities
                            .for_touch_id
                            .insert(event.id, Vec::new());
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn track_virtual_gamepad(
    gamepad_trackers: Query<&GamepadTracker>,
    mut commands: Commands,
    mut touch_events: EventReader<TouchInput>,
    mut touch_markers: Query<(&mut Style, &Label, &mut Controller)>,
    mut knob_trail_entities: ResMut<KnobTrailEntities>,
    mut texts: Query<&mut Text, With<DebugText>>,
) {
    for event in touch_events.iter() {
        match event.phase {
            TouchPhase::Moved => {
                for tracker in &gamepad_trackers {
                    if let Some(touch_id) = tracker.touch_id {
                        if touch_id != event.id {
                            continue;
                        }

                        for (mut style, marker, mut controller) in &mut touch_markers {
                            if marker.touch_id != event.id {
                                continue;
                            }
                            style.left = Val::Px(event.position.x - GAMEPAD_TOUCH_SIZE / 2.);
                            style.top = Val::Px(event.position.y - GAMEPAD_TOUCH_SIZE / 2.);

                            controller.touch_position = event.position;

                            // Add knob trails
                            let touch_drag_distance =
                                event.position.distance(controller.start_position);
                            let dot_spacing = distance_between_dots(24., touch_drag_distance);
                            let num_of_dots = (touch_drag_distance / dot_spacing) as usize;

                            for mut text in &mut texts {
                                text.sections[0].value = format!("Distance: {touch_drag_distance:.2} | Spacing: {dot_spacing:.2} | Dots count: {num_of_dots}");
                            }

                            if let Some(entities) =
                                knob_trail_entities.for_touch_id.get_mut(&event.id)
                            {
                                if entities.len() < num_of_dots {
                                    while entities.len() < num_of_dots {
                                        let entity = commands
                                            .spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(GAMEPAD_TRAIL_DOT_SIZE),
                                                        height: Val::Px(GAMEPAD_TRAIL_DOT_SIZE),
                                                        left: Val::Px(
                                                            controller.start_position.x
                                                                - GAMEPAD_TRAIL_DOT_SIZE / 2.,
                                                        ),
                                                        top: Val::Px(
                                                            controller.start_position.y
                                                                - GAMEPAD_TRAIL_DOT_SIZE / 2.,
                                                        ),
                                                        position_type: PositionType::Absolute,
                                                        border: UiRect::all(Val::Px(5.0)),
                                                        ..default()
                                                    },
                                                    background_color: Color::GRAY.into(),
                                                    border_color: BorderColor(Color::WHITE),
                                                    ..default()
                                                },
                                                Label { touch_id: event.id },
                                                KnobTrailDot,
                                            ))
                                            .id();

                                        entities.push(entity);
                                    }
                                } else if entities.len() > num_of_dots {
                                    while entities.len() > num_of_dots {
                                        if let Some(entity) = entities.pop() {
                                            commands.entity(entity).despawn();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn arrange_knob_trail_dots(
    gamepad_trackers: Query<&GamepadTracker>,
    knob_trail_entities: Res<KnobTrailEntities>,
    touch_markers: Query<(&Style, &Label, &Controller)>,
    mut knob_trail_dots: Query<(&mut Style, &Label), (With<KnobTrailDot>, Without<Controller>)>,
) {
    for tracker in &gamepad_trackers {
        if let Some(touch_id) = tracker.touch_id {
            for (touch_marker_style, touch_marker_label, touch_marker_controller) in &touch_markers
            {
                if touch_marker_label.touch_id != touch_id {
                    continue;
                }

                if let Some(entities) = knob_trail_entities.for_touch_id.get(&touch_id) {
                    let dots_count = entities.len();
                    let touch_drag_distance = touch_marker_controller
                        .start_position
                        .distance(touch_marker_controller.touch_position);
                    let dot_spacing = distance_between_dots(24., touch_drag_distance);
                    let drag_inverse_vector = touch_marker_controller.start_position
                        - touch_marker_controller.touch_position;

                    let angle = drag_inverse_vector.y.atan2(drag_inverse_vector.x);
                    let angle_sin = angle.sin();
                    let angle_cos = angle.cos();

                    for (i, entity) in entities.iter().enumerate() {
                        if let Ok((mut trail_dot_style, trail_dot_label)) =
                            knob_trail_dots.get_mut(*entity)
                        {
                            let magnitude = (i + 1) as f32 * dot_spacing;
                            let trail_dot_offset =
                                Vec2::new(magnitude * angle_cos, magnitude * angle_sin);
                            let trail_dot_position = touch_marker_controller.touch_position + trail_dot_offset;

                            trail_dot_style.left = Val::Px(trail_dot_position.x  - GAMEPAD_TRAIL_DOT_SIZE / 2.);
                            trail_dot_style.top = Val::Px(trail_dot_position.y  - GAMEPAD_TRAIL_DOT_SIZE / 2.);
                        }
                    }
                }
            }
        }
    }
}

pub fn release_virtual_gamepad(// mut commands: Commands,
    // mut touch_events: EventReader<TouchInput>,
    // mut gamepads: Query<(&mut GamepadPosition, &mut GamepadTracker)>,
    // touch_markers: Query<(Entity, &Label)>,
    // anchor_markers: Query<(Entity, &Controller)>
) {
    // for event in touch_events.iter() {
    //     match event.phase {
    //         TouchPhase::Ended | TouchPhase::Canceled => {
    //             for (mut position, mut tracker) in &mut gamepads {
    //                 if let Some(touch_id) = tracker.touch_id {
    //                     if touch_id != event.id {
    //                         continue;
    //                     }
    //                     position.anchor = None;
    //                     tracker.touch_id = None;
    //                 }
    //             }
    //
    //             for (entity, marker) in &touch_markers {
    //                 if marker.touch_id == event.id {
    //                     commands.entity(entity).despawn_recursive();
    //                 }
    //             }
    //
    //             for (entity, marker) in &anchor_markers {
    //                 if marker.touch_id == event.id {
    //                     commands.entity(entity).despawn_recursive();
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    // }
}

pub fn init_virtual_joystick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning joystick");
    // commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            position_type: PositionType::Absolute,
            left: Val::Px(96.),
            bottom: Val::Px(96.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::ANTIQUE_WHITE.into(),
        ..default()
    });
}
