pub mod components;
mod systems;

use crate::lifespan::systems::{age, kill};
use bevy::prelude::*;

pub struct LifespanPlugin;

impl Plugin for LifespanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (age, kill));
    }
}
