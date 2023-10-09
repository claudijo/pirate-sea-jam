use crate::components::cannon::{Cannon, CannonBall};
use crate::components::ship::Ship;
use crate::resources::assets::ModelAssets;
use crate::resources::wave_machine::WaveMachine;
use bevy::prelude::*;
use bevy_rapier3d::prelude::ColliderMassProperties::Density;
use bevy_rapier3d::prelude::Velocity;
use bevy_rapier3d::prelude::*;
use rand::Rng;

pub fn fire_cannons(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    cannons: Query<(&GlobalTransform, &Cannon)>,
    rigs: Query<&Velocity, With<Ship>>,
) {
    for (global_transform, cannon) in &cannons {
        if cannon.is_lit {
            let mut rng = rand::thread_rng();

            if let Some(rig_entity) = cannon.rig {
                if let Ok(rig_velocity) = rigs.get(rig_entity) {
                    commands.spawn((
                        SceneBundle {
                            scene: model_assets.scene_handles["cannon_ball"].clone(),
                            transform: Transform::from_translation(global_transform.translation()),
                            ..default()
                        },
                        CannonBall,
                        RigidBody::Dynamic,
                        ExternalImpulse {
                            impulse: global_transform.left()
                                * 20.
                                * cannon.power
                                * rng.gen_range(0.9..1.1),
                            ..default()
                        },
                        Collider::ball(0.3),
                        Density(10.),
                        Velocity {
                            linvel: rig_velocity.linvel,
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

pub fn despawn_cannon_ball(
    mut commands: Commands,
    cannon_balls: Query<(Entity, &GlobalTransform), With<CannonBall>>,
    wave_machine: Res<WaveMachine>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();
    for (entity, global_transform) in &cannon_balls {
        let translation = global_transform.translation();
        let water_height = wave_machine.surface_height(translation, elapsed_time);
        if translation.y + 2. < water_height {
            commands.entity(entity).despawn_recursive();
        }
    }
}