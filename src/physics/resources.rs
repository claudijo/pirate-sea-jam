use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec3);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(Vec3::NEG_Y * 15.)
    }
}

#[derive(Resource)]
pub struct WaterDensity(pub f32);

impl Default for WaterDensity {
    fn default() -> Self {
        // 1000 kg per cubic meter.
        WaterDensity(1000.)
    }
}

#[derive(Resource)]
pub struct AirDensity(pub f32);

impl Default for crate::physics::resources::AirDensity {
    fn default() -> Self {
        // 1.2 kg per cubic meter.
        AirDensity(1.2)
    }
}
