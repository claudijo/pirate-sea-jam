use crate::orbiting_camera::resources::FocalPoint;
use crate::sky_box::systems::{spawn_sky_box, sync_sky_box_center_offset};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::*;

mod components;
mod systems;

pub struct SkyBoxPlugin;

impl Plugin for SkyBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sky_box);

        app.add_systems(
            Update,
            sync_sky_box_center_offset.run_if(resource_changed::<FocalPoint>),
        );
    }
}
