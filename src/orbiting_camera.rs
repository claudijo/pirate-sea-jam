pub mod events;
pub mod resources;
mod systems;

use crate::game_state::states::GameState;
use crate::orbiting_camera::events::OrbitMotion;
use crate::orbiting_camera::systems::orbit;
use bevy::prelude::*;

pub struct OrbitingCameraPlugin;

impl Plugin for OrbitingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrbitMotion>();

        app.add_systems(Update, orbit.run_if(in_state(GameState::InGame)));
    }
}
