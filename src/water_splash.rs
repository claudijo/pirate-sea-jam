pub mod components;
mod systems;

use crate::game_state::states::GameState;
use crate::water_splash::systems::update_water_splash_intensity;
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

pub struct WaterSplashPlugin;

impl Plugin for WaterSplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            update_water_splash_intensity.run_if(in_state(GameState::InGame)),
        );
    }
}
