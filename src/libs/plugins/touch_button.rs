use bevy::input::touch::{ForceTouch, Touch};
use bevy::prelude::*;
use bevy::input::touch::TouchPhase;
use bevy::ui::FocusPolicy;

#[derive(Component, Default, Deref, DerefMut)]
pub struct TouchId(Option<u64>);

#[derive(Event)]
pub struct TouchInteraction {
    pub phase: TouchPhase,
    pub position: Vec2,
    pub force: Option<ForceTouch>,
    pub id: u64,
    pub source: Entity,
}

impl TouchInteraction {
    fn new(touch_input: &TouchInput, source: Entity) -> Self {
        TouchInteraction {
            phase: touch_input.phase,
            position: touch_input.position,
            force: touch_input.force,
            id: touch_input.id,
            source,
        }
    }
}

#[derive(Bundle)]
pub struct TouchButtonBundle {
    pub focus_policy: FocusPolicy,
    pub node: Node,
    pub touch_id: TouchId,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,
}


impl Default for TouchButtonBundle {
    fn default() -> Self {
        Self {
            focus_policy: FocusPolicy::Block,
            node: Default::default(),
            touch_id: Default::default(),
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

pub fn handle_touch_event(
    mut touch_input_event_reader: EventReader<TouchInput>,
    mut touch_interaction_event_writer: EventWriter<TouchInteraction>,
    mut touch_button_query: Query<(Entity, &Node, &GlobalTransform, &mut TouchId)>
) {
    for touch_input_event in touch_input_event_reader.iter() {
        for (entity, node, global_transform, mut touch_id) in &mut touch_button_query {
            let mut fire_event = false;

            match touch_input_event.phase {
                TouchPhase::Started => {
                    let rect = node.logical_rect(global_transform);
                    let is_finger_over = rect.contains(touch_input_event.position);

                    if is_finger_over && (**touch_id).is_none() {
                        **touch_id = Some(touch_input_event.id);
                        fire_event = true;
                    }
                }

                TouchPhase::Moved => {
                    if let Some(id) = **touch_id {
                        if id == touch_input_event.id {
                            fire_event = true;
                        }
                    }
                }

                // Ended or Cancelled
                _ => {
                    if let Some(id) = **touch_id {
                        if id == touch_input_event.id {
                            (**touch_id) = None;
                            fire_event = true;
                        }
                    }
                }
            }

            if fire_event {
                touch_interaction_event_writer.send(TouchInteraction::new(&touch_input_event, entity));
            }
        }
    }
}

pub struct TouchButtonPlugin;

impl Plugin for TouchButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TouchInteraction>();

        app.add_systems(Update, handle_touch_event);
    }
}
