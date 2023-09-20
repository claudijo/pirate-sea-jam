use crate::components::ship::{Rudder, Helm};
use bevy::prelude::*;

pub fn turn_ship(mut ships: Query<&Helm>) {
    
}

pub fn rotate_helm(
    rudders: Query<&Rudder>,
    mut helms: Query<&mut Transform, With<Helm>>,
) {
    for rudder in &rudders {
        for mut transform in &mut helms {
            transform.rotation = Quat::from_rotation_z(rudder.angle.to_radians() * 20.);
        }
    }
    
}
