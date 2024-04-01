use crate::wind::resources::Wind;
use bevy::prelude::*;

pub mod resources;

pub struct WindPlugin;

impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Wind(Vec3::new(10., 0., 0.)));
    }
}
