use bevy::prelude::*;
use crate::components::button::ReleasableButton;
use crate::events::button::ButtonReleasedEvent;

pub fn react_to_button_release(
    mut interaction_query: Query<(Entity, &Interaction, &mut ReleasableButton), Changed<Interaction>>,
    mut event_writer: EventWriter<ButtonReleasedEvent>,
) {
    for (entity, interaction, mut releasable_button) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if releasable_button.last_state == Interaction::Pressed {
                    event_writer.send(ButtonReleasedEvent(entity));
                }
            }
            _ => {}
        }
        releasable_button.last_state = *interaction;
    }
}