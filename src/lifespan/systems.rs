use crate::lifespan::components::Lifespan;
use bevy::prelude::*;

pub fn age(mut lifespan_query: Query<&mut Lifespan>, time: Res<Time>) {
    for mut lifespan in &mut lifespan_query {
        lifespan.ttl.tick(time.delta());
    }
}

pub fn kill(mut commands: Commands, mut lifespan_query: Query<(Entity, &mut Lifespan)>) {
    for (entity, lifespan) in &mut lifespan_query {
        if lifespan.ttl.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
