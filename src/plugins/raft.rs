use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::plugins::buoy::{BuoyBundle, CORK_DENSITY};

#[derive(Component)]
pub struct Raft {}

pub fn spawn_raft(
    mut commands: Commands,
) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(8., 15., 5.)),
        BuoyBundle {
            collider: Collider::ball(0.5),
            ..default()
        },
    ));
}

pub struct RaftPlugin;

impl Plugin for RaftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_raft);
    }
}