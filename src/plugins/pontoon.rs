use crate::systems::fluid_dynamics;
use bevy::prelude::*;

pub struct PontoonPlugin;

impl Plugin for PontoonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fluid_dynamics::buoyancy);
    }
}
