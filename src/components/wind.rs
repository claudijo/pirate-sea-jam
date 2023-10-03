use bevy::prelude::*;

#[derive(Component)]
pub struct Wind {
    pub direction: Vec3,
}

impl Default for Wind {
    fn default() -> Self {
        Self { direction: Vec3::X }
    }
}
