use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::components::ship::ShipFlag;
use crate::game_state::GameState;
use crate::plugins::assets::ModelAssets;
use crate::plugins::buoy::{BALSA_DENSITY, BuoyBundle, CORK_DENSITY, OAK_DENSITY, PINE_DENSITY};
use crate::resources::wave::Wave;

#[derive(Component)]
pub struct Raft;

pub fn spawn_raft(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
) {
    let spawn_at = Vec3::new(10., 6., 10.);

    let parent_entity = commands.spawn((
        // The rigid body
        TransformBundle::from(Transform::from_translation(spawn_at)),
        RigidBody::Dynamic,
        VisibilityBundle { ..default() }, // Necessary to display child scene bundle
        Raft,
    )).with_children(|child_builder| {
        // Collider mast
        child_builder.spawn((
            TransformBundle::from(Transform::from_xyz(0., 2., 0.)),
            Collider::cuboid(0.4, 4., 0.4),
            ColliderDensity(BALSA_DENSITY),
        ));

        // Collider base
        child_builder.spawn((
            TransformBundle::default(),
            Collider::cuboid(1.4, 0.6, 1.4),
            ColliderDensity(CORK_DENSITY),
        ));

        // Models
        child_builder.spawn(SceneBundle {
            transform: Transform::default(),
            scene: model_assets.scene_handles["raft_with_mast.glb"].clone(),
            ..default()
        });
    })
        .id();

    // Spawn children that need a reference to the parent entity
    commands
        .entity(parent_entity)
        .with_children(|child_builder| {
            child_builder.spawn((
                SceneBundle {
                    scene: model_assets.scene_handles["pirate_flag.glb"].clone(),
                    transform: Transform::from_xyz(0.0829, 3.2132, 0.0581),
                    ..default()
                },
                ShipFlag { rig: parent_entity },
            ));
        });

    const DEFAULT_RADIUS: f32 = 0.5;
    let buoy_configs = [
        (Vec3::new(-0.8, 0., 0.8), DEFAULT_RADIUS),
        (Vec3::new(0.8, 0., 0.8), DEFAULT_RADIUS),
        (Vec3::new(-0.8, 0., -0.8), DEFAULT_RADIUS),
        (Vec3::new(0.8, 0., -0.8), DEFAULT_RADIUS),
        (Vec3::new(0., 2., 0.), 0.4),
    ];

    for (position, radius) in buoy_configs {
        let child_pontoon = commands.spawn((
            BuoyBundle::from_transform(Transform::from_translation(spawn_at + position))
                .with_radius(radius),
        )).id();

        commands.spawn(
            FixedJoint::new(parent_entity, child_pontoon).with_local_anchor_1(position),
        );
    }
}

pub struct RaftPlugin;

impl Plugin for RaftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SplashScreen),
            spawn_raft,
        );
    }
}