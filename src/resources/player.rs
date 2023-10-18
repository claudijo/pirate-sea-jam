use bevy::prelude::*;

#[derive(PartialEq, Eq, Resource, Default)]
pub enum InputDevice {
    #[default]
    Mouse,
    Touch,
}
