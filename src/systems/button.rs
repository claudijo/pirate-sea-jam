use crate::components::button::ReleasableTouchButton;
use crate::events::button::ButtonReleased;
use bevy::prelude::*;

pub fn react_to_touch_button_release(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut ReleasableTouchButton),
        Changed<Interaction>,
    >,
    mut button_release_event_writer: EventWriter<ButtonReleased>,
) {
    for (entity, interaction, mut releasable_button) in &mut interaction_query {
        if releasable_button.last_state == Interaction::Pressed {
            button_release_event_writer.send(ButtonReleased(entity));
        }
        releasable_button.last_state = *interaction;
    }
}
