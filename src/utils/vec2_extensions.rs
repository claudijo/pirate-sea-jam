use bevy::prelude::*;
use std::f32::consts::E;

pub trait Vec2Ext {
    #[allow(dead_code)]
    fn extend_with_y(self, y: f32) -> Vec3;
    #[allow(dead_code)]
    fn damp(self, rhs: Self, lambda: f32, delta_time: f32) -> Self;
}

impl Vec2Ext for Vec2 {
    fn extend_with_y(self, y: f32) -> Vec3 {
        Vec3::new(self.x, y, self.y)
    }

    // lambda has range between `0` and infinity, will approach rhs
    // See https://www.rorydriscoll.com/2016/03/07/frame-rate-independent-damping-using-lerp/
    fn damp(self, rhs: Self, lambda: f32, delta_time: f32) -> Self {
        self.lerp(rhs, 1. - E.powf(-lambda * delta_time))
    }
}
