use bevy::prelude::*;

#[derive(Component)]
pub struct Hull {
    pub stability: f32,
    pub manoeuvrability: f32,
    pub max_speed: f32,
}

impl Default for Hull {
    fn default() -> Self {
        Self {
            stability: 100.,
            manoeuvrability: 30.,
            max_speed: 100.,
        }
    }
}

#[derive(Component)]
pub struct Rudder {
    pub angle: f32,
}

impl Default for Rudder {
    fn default() -> Self {
        Self {
            angle: 0.,
        }
    }
}

#[derive(Component)]
pub struct Helm;
