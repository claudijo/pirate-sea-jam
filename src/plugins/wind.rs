use crate::components::wind::Wind;
use crate::game_state::GameState;
use bevy::prelude::*;

pub struct WindPlugin;

impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_wind);
    }
}

fn spawn_wind(mut commands: Commands) {
    commands.spawn(Wind { ..default() });
}
