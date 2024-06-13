use crate::lifespan::components::Lifespan;
use crate::particles::components::ParticleEmitter;
use crate::physics::bundles::{ParticlePhysicsBundle, SpindlePhysicsBundle};
use crate::physics::components::LinearVelocity;
use bevy::prelude::*;

pub fn emit_particles(
    mut commands: Commands,
    mut particle_emitter_query: Query<(&mut ParticleEmitter, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (mut emitter, global_transform) in &mut particle_emitter_query {
        emitter.rate.tick(time.delta());
        if emitter.rate.just_finished() {
            let global_velocity = global_transform
                .affine()
                .transform_vector3(emitter.particle_velocity);

            let material = materials.add(StandardMaterial {
                base_color: emitter.particle_color,
                unlit: true,
                ..default()
            });
            // let mesh = meshes.add(Triangle2d::default().mesh().scaled_by(Vec3::splat(emitter.particle_size + emitter.particle_scale_variance * (2.0 * rand::random::<f32>() - 1.0))));
            let mesh = meshes.add(
                Sphere::default()
                    .mesh()
                    .ico(0)
                    .unwrap()
                    .scaled_by(Vec3::splat(
                        emitter.particle_size
                            + emitter.particle_scale_variance * (2.0 * rand::random::<f32>() - 1.0),
                    )),
            );

            for _i in 0..emitter.amount_per_burst {
                commands
                    .spawn((
                        PbrBundle {
                            mesh: mesh.clone(),
                            material: material.clone(),
                            transform: Transform::from_translation(
                                Vec3::new(
                                    emitter.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                                    emitter.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                                    emitter.position_variance * (2.0 * rand::random::<f32>() - 1.0),
                                ) + global_transform.translation(),
                            ),
                            ..default()
                        },
                        ParticlePhysicsBundle {
                            linear_velocity: LinearVelocity(global_velocity),
                            ..default()
                        },
                        SpindlePhysicsBundle {
                            ..default()
                        },
                        Lifespan {
                            ttl: Timer::from_seconds(emitter.particle_lifetime, TimerMode::Once),
                        },
                    ));
            }
        }
    }
}
