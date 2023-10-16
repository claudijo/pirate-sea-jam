use crate::components::wind::Wind;
use bevy::prelude::*;

pub fn spawn_wind(mut commands: Commands) {
    commands.spawn(Wind { ..default() });
}
