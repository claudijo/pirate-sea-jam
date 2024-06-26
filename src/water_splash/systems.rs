use crate::ocean::resources::Wave;
use crate::particles::components::ParticleEmitter;
use crate::water_splash::components::WaterSplasher;
use bevy::prelude::*;

pub fn update_water_splash_intensity(
    mut particle_emitter_query: Query<(&mut ParticleEmitter, &WaterSplasher, &GlobalTransform)>,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed_seconds();

    for (mut particle_emitter, water_splasher, global_transform) in &mut particle_emitter_query {
        let water_height = wave.height(global_transform.translation(), wave.configs, elapsed_time);
        let submerged =
            (global_transform.translation().y - water_height - water_splasher.max_depth)
                / (-2. * water_splasher.max_depth);

        particle_emitter.amount_per_burst = (20. * submerged) as usize;
    }
}
