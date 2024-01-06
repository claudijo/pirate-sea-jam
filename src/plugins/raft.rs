use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::plugins::buoy::{BuoyBundle, CORK_DENSITY};

#[derive(Component)]
pub struct Raft {}

pub fn spawn_raft(
    mut commands: Commands,
) {
    commands.spawn((
        BuoyBundle::from_transform(Transform::from_xyz(8., 15., 5.)).with_radius(0.5),
    ));
}

pub struct RaftPlugin;

impl Plugin for RaftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_raft);
    }
}