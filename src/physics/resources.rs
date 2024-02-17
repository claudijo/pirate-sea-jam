use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec3);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(Vec3::NEG_Y * 15.)
    }
}
