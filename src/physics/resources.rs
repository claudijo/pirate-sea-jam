use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec3);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(Vec3::NEG_Y * 15.)
    }
}

#[derive(Resource)]
pub struct LiquidDensity(pub f32);

impl Default for LiquidDensity {
    fn default() -> Self {
        // 1000 kg per cubic meter.
        LiquidDensity(1000.)
    }
}
