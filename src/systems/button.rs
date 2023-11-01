use crate::components::button::ReleasableTouchButton;
use crate::events::button::ButtonReleasedEvent;
use bevy::prelude::*;

pub fn react_to_touch_button_release(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut ReleasableTouchButton),
        Changed<Interaction>,
    >,
    mut event_writer: EventWriter<ButtonReleasedEvent>,
) {
    for (entity, interaction, mut releasable_button) in &mut interaction_query {
        if releasable_button.last_state == Interaction::Pressed {
            event_writer.send(ButtonReleasedEvent(entity));
        }
        releasable_button.last_state = *interaction;
    }
}
