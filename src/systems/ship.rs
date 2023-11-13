use crate::components::cannon::{Aim, Cannon};
use crate::components::pontoon::Pontoon;
use crate::components::ship::{
    PlayerShip, Ship, ShipBooster, ShipFlag, ShipHelm, ShipRudder, ShipSail,
};
use crate::events::game::RestartGameEvent;
use crate::plugins::assets::ModelAssets;
use crate::resources::despawn::ShipDespawnEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Resource)]
pub struct StartAimCannonAnimationClips {
    pub handles: HashMap<&'static str, Handle<AnimationClip>>,
}

#[derive(Resource)]
pub struct EndAimCannonAnimationClips {
    pub handles: HashMap<&'static str, Handle<AnimationClip>>,
}

const PORT_BACK_CANNON_TAG: &str = "Port back cannon";
const PORT_FRONT_CANNON_TAG: &str = "Port front cannon";
const STARBOARD_BACK_CANNON_TAG: &str = "Starboard back cannon";
const STARBOARD_FRONT_CANNON_TAG: &str = "Starboard front cannon";

pub fn register_stop_aim_cannon_animations(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let keyframe_timestamps = vec![0.0];

    let reset_tilt_port_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![Quat::from_rotation_y(PI)]),
    };

    let reset_tilt_starboard_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![Quat::from_rotation_y(0.)]),
    };

    let stretch_and_squash_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: vec![0.0, 0.06, 0.18, 0.2],
        keyframes: Keyframes::Scale(vec![
            Vec3::new(1., 1., 1.),
            Vec3::new(0.6, 1.6, 1.6),
            Vec3::new(1.1, 0.9, 0.9),
            Vec3::new(1., 1., 1.),
        ]),
    };

    let mut port_back_cannon_animation = AnimationClip::default();
    let mut port_front_cannon_animation = AnimationClip::default();
    let mut starboard_back_cannon_animation = AnimationClip::default();
    let mut starboard_front_cannon_animation = AnimationClip::default();

    // Reset tilt
    port_back_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(PORT_BACK_CANNON_TAG)],
        },
        reset_tilt_port_cannon_animation_curve.clone(),
    );

    port_front_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(PORT_FRONT_CANNON_TAG)],
        },
        reset_tilt_port_cannon_animation_curve.clone(),
    );

    starboard_back_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(STARBOARD_BACK_CANNON_TAG)],
        },
        reset_tilt_starboard_cannon_animation_curve.clone(),
    );

    starboard_front_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(STARBOARD_FRONT_CANNON_TAG)],
        },
        reset_tilt_starboard_cannon_animation_curve.clone(),
    );

    // Stretch and squash
    port_back_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(PORT_BACK_CANNON_TAG)],
        },
        stretch_and_squash_cannon_animation_curve.clone(),
    );

    port_front_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(PORT_FRONT_CANNON_TAG)],
        },
        stretch_and_squash_cannon_animation_curve.clone(),
    );

    starboard_back_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(STARBOARD_BACK_CANNON_TAG)],
        },
        stretch_and_squash_cannon_animation_curve.clone(),
    );

    starboard_front_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(STARBOARD_FRONT_CANNON_TAG)],
        },
        stretch_and_squash_cannon_animation_curve.clone(),
    );

    let mut animation_clip_handles = HashMap::new();

    animation_clip_handles.insert(
        PORT_BACK_CANNON_TAG,
        animations.add(port_back_cannon_animation),
    );
    animation_clip_handles.insert(
        PORT_FRONT_CANNON_TAG,
        animations.add(port_front_cannon_animation),
    );
    animation_clip_handles.insert(
        STARBOARD_BACK_CANNON_TAG,
        animations.add(starboard_back_cannon_animation),
    );
    animation_clip_handles.insert(
        STARBOARD_FRONT_CANNON_TAG,
        animations.add(starboard_front_cannon_animation),
    );

    commands.insert_resource(EndAimCannonAnimationClips {
        handles: animation_clip_handles,
    });
}

pub fn register_start_aim_cannon_animations(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let keyframe_timestamps = vec![0.0, 0.6, 1.2];

    let tilt_port_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![
            Quat::from_rotation_y(PI) * Quat::from_rotation_z(0.),
            Quat::from_rotation_y(PI) * Quat::from_rotation_z(-30_f32.to_radians()),
            Quat::from_rotation_y(PI) * Quat::from_rotation_z(0.),
        ]),
    };

    let tilt_starboard_cannon_animation_curve = VariableCurve {
        keyframe_timestamps: keyframe_timestamps.clone(),
        keyframes: Keyframes::Rotation(vec![
            Quat::from_rotation_z(0.),
            Quat::from_rotation_z(-30_f32.to_radians()),
            Quat::from_rotation_z(0.),
        ]),
    };

    let mut port_back_cannon_animation = AnimationClip::default();
    let mut port_front_cannon_animation = AnimationClip::default();
    let mut starboard_back_cannon_animation = AnimationClip::default();
    let mut starboard_front_cannon_animation = AnimationClip::default();

    port_back_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(PORT_BACK_CANNON_TAG)],
        },
        tilt_port_cannon_animation_curve.clone(),
    );

    port_front_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(PORT_FRONT_CANNON_TAG)],
        },
        tilt_port_cannon_animation_curve.clone(),
    );

    starboard_back_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(STARBOARD_BACK_CANNON_TAG)],
        },
        tilt_starboard_cannon_animation_curve.clone(),
    );

    starboard_front_cannon_animation.add_curve_to_path(
        EntityPath {
            parts: vec![Name::new(STARBOARD_FRONT_CANNON_TAG)],
        },
        tilt_starboard_cannon_animation_curve.clone(),
    );

    let mut animation_clip_handles = HashMap::new();

    animation_clip_handles.insert(
        PORT_BACK_CANNON_TAG,
        animations.add(port_back_cannon_animation),
    );
    animation_clip_handles.insert(
        PORT_FRONT_CANNON_TAG,
        animations.add(port_front_cannon_animation),
    );
    animation_clip_handles.insert(
        STARBOARD_BACK_CANNON_TAG,
        animations.add(starboard_back_cannon_animation),
    );
    animation_clip_handles.insert(
        STARBOARD_FRONT_CANNON_TAG,
        animations.add(starboard_front_cannon_animation),
    );

    commands.insert_resource(StartAimCannonAnimationClips {
        handles: animation_clip_handles,
    });
}

pub fn spawn_ship(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    mut ship_despawn: ResMut<ShipDespawnEntities>,
) {
    let parent_entity = commands
        // Rigid body
        .spawn((
            TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
            RigidBody::Dynamic,
            VisibilityBundle { ..default() }, // Necessary to display child scene bundle
            ExternalImpulse { ..default() },
            ExternalForce { ..default() },
            Velocity { ..default() },
            Damping {
                angular_damping: 20.,
                linear_damping: 4.,
            },
            PlayerShip,
            Ship { ..default() },
            ShipBooster { ..default() },
            ShipRudder { ..default() },
        ))
        .with_children(|child_builder| {
            // Colliders
            child_builder.spawn((Collider::cuboid(0.8, 0.5, 2.),));
        })
        .id();

    // Spawn children that need a reference to the parent entity
    commands
        .entity(parent_entity)
        .with_children(|child_builder| {
            // Models
            child_builder
                .spawn((SceneBundle {
                    scene: model_assets.scene_handles["medium_hull.glb"].clone(),
                    transform: Transform::from_xyz(0., -0.5, 0.),
                    ..default()
                },))
                .with_children(|child_builder| {
                    child_builder.spawn((
                        ShipHelm,
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_helm.glb"].clone(),
                            transform: Transform::from_xyz(0., 5.5806, -1.0694),
                            ..default()
                        },
                    ));

                    child_builder.spawn((
                        ShipSail,
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_pirate_sail.glb"].clone(),
                            transform: Transform::from_xyz(0., 2.3248, 1.3574),
                            ..default()
                        },
                    ));

                    child_builder.spawn((
                        ShipFlag { rig: parent_entity },
                        SceneBundle {
                            scene: model_assets.scene_handles["medium_flag.glb"].clone(),
                            transform: Transform::from_xyz(0., 9.38793, 1.35834),
                            ..default()
                        },
                    ));

                    #[rustfmt::skip]
                        let cannons = [
                        ([1.1769, 1.4593, -0.5485], PI, Name::new(PORT_BACK_CANNON_TAG)), // Port back cannon
                        ([1.13846, 1.54822, 1.54781], PI, Name::new(PORT_FRONT_CANNON_TAG)), // Port front cannon
                        ([-1.1769, 1.4593, -0.5485], 0., Name::new(STARBOARD_BACK_CANNON_TAG)), // Starboard back cannon
                        ([-1.13846, 1.54822, 1.54781], 0., Name::new(STARBOARD_FRONT_CANNON_TAG)), // Starboard front cannon
                    ];

                    for (cannon_transform, cannon_y_rotation, name) in cannons {
                        child_builder.spawn((
                            Aim { ..default() },
                            Cannon {
                                rig: parent_entity,
                                power: 1.,
                            },
                            SceneBundle {
                                scene: model_assets.scene_handles["medium_canon.glb"].clone(),
                                transform: Transform::from_translation(Vec3::from_array(
                                    cannon_transform,
                                ))
                                .with_rotation(Quat::from_rotation_y(cannon_y_rotation)),
                                ..default()
                            },
                            AnimationPlayer::default(),
                            name,
                        ));
                    }
                });
        });

    let pontoon_positions = [
        [-0.8, 0., 2.],
        [0.8, 0., 2.],
        [-1., 0., 0.],
        [1., 0., 0.],
        [-0.6, 0., -2.],
        [0.6, 0., -2.],
    ];

    let pontoon_radius = 0.5;

    for pontoon_position in pontoon_positions {
        let position = Vec3::from_array(pontoon_position);
        let joint = FixedJointBuilder::new().local_anchor1(position);

        let child_pontoon = commands
            .spawn((
                TransformBundle::from(Transform::from_translation(position)),
                RigidBody::Dynamic,
                Collider::ball(pontoon_radius),
                ExternalForce { ..default() },
                Damping { ..default() },
                Velocity { ..default() },
                CollisionGroups::new(Group::NONE, Group::NONE),
                Pontoon {
                    radius: pontoon_radius,
                    ..default()
                },
                ImpulseJoint::new(parent_entity, joint),
                Name::new("pontoon"),
            ))
            .id();

        // Need to add pontoon to registry for later despawn
        ship_despawn.entities.push(child_pontoon);
    }
}

pub fn reset_ship(
    model_assets: Res<ModelAssets>,
    ship_query: Query<Entity, With<Ship>>,
    mut commands: Commands,
    mut ship_despawn: ResMut<ShipDespawnEntities>,
    mut restart_game_event_reader: EventReader<RestartGameEvent>,
) {
    if restart_game_event_reader.is_empty() {
        return;
    }
    restart_game_event_reader.clear();

    // Note that some joint related child entities seem to be missing from the normal
    // parent-child-hierarchy when despawning, so those are registered and handled "manually".
    // (See https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier3d/examples/joints_despawn3.rs)
    for parent in &ship_query {
        for entity in &ship_despawn.entities {
            commands.entity(*entity).despawn();
        }

        ship_despawn.entities.clear();
        commands.entity(parent).despawn_recursive();
    }

    // Spawn new ship
    spawn_ship(commands, model_assets, ship_despawn);
}
