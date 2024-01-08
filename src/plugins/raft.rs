use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::components::ship::ShipFlag;
use crate::events::game::RestartGameEvent;
use crate::game_state::GameState;
use crate::plugins::assets::ModelAssets;
use crate::plugins::buoy::{BALSA_DENSITY, BuoyBundle, CORK_DENSITY, OAK_DENSITY, PINE_DENSITY};
use crate::resources::despawn::ShootingTargetDespawnEntities;

#[derive(Resource, Default)]
struct DespwawnEntities {
    entities: Vec<Entity>,
}

#[derive(Component)]
pub struct Raft;

pub fn spawn_raft(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    mut despawn_entities: ResMut<DespwawnEntities>,
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
        let child_pontoon = commands.spawn(
            BuoyBundle::from_transform(Transform::from_translation(spawn_at + position))
                .with_radius(radius),
        ).id();

        let joint = commands.spawn(
            FixedJoint::new(parent_entity, child_pontoon).with_local_anchor_1(position),
        ).id();

        despawn_entities.entities.push(child_pontoon);
        despawn_entities.entities.push(joint);
    }
}

fn reset_raft(
    mut commands: Commands,
    raft_query: Query<Entity, With<Raft>>,
    mut restart_game_event_reader: EventReader<RestartGameEvent>,
    model_assets: Res<ModelAssets>,
    mut despawn_entities: ResMut<DespwawnEntities>,
) {
    if restart_game_event_reader.is_empty() {
        return;
    }

    restart_game_event_reader.clear();

    // Remove current raft
    for parent in &raft_query {
        commands.entity(parent).despawn_recursive();
    }

    for entity in &despawn_entities.entities {
        commands.entity(*entity).despawn();
    }

    despawn_entities.entities.clear();

    // Spawn new raft
    spawn_raft(commands, model_assets, despawn_entities);
}

pub struct RaftPlugin;

impl Plugin for RaftPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DespwawnEntities::default());

        app.add_systems(
            OnEnter(GameState::SplashScreen),
            spawn_raft,
        );

        app.add_systems(
            Update,
            reset_raft.run_if(in_state(GameState::InGame)),
        );
    }
}